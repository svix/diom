use opentelemetry_sdk::metrics::{
    data::{AggregatedMetrics, MetricData, ResourceMetrics},
    exporter::PushMetricExporter,
};
use parking_lot::Mutex;
use schemars::JsonSchema;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    hash::Hasher,
    sync::Arc,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InternHandle(usize);

#[derive(Default)]
/// Simple string interner
///
/// Replace your long-lived strings with Copy-able InternHandles
struct StringStore {
    inner: indexmap::IndexSet<String>,
}

impl StringStore {
    /// Intern the given string
    ///
    /// If it's already stored inside this structure, return a handle
    /// to the existing record; otherwise, allocate a new owned record
    /// and return a handle to it
    fn intern<S>(&mut self, s: S) -> InternHandle
    where
        S: Into<String> + AsRef<str>,
    {
        let index = if let Some(index) = self.inner.get_index_of(s.as_ref()) {
            index
        } else {
            self.inner.insert_full(s.into()).0
        };
        InternHandle(index)
    }

    /// Turn an InternHandle back into a str
    fn redeem(&self, handle: InternHandle) -> Option<&str> {
        self.inner.get_index(handle.0).map(|s| s.as_str())
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum MetricDatum {
    U64(u64),
    I64(i64),
    F64(f64),
}

impl From<MetricDatum> for f64 {
    fn from(value: MetricDatum) -> Self {
        match value {
            MetricDatum::F64(f) => f,
            MetricDatum::U64(u) => u as _,
            MetricDatum::I64(i) => i as _,
        }
    }
}

impl From<f64> for MetricDatum {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl From<i64> for MetricDatum {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<u64> for MetricDatum {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

type AttributeList = BTreeSet<(InternHandle, InternHandle)>;

#[derive(Debug)]
pub(crate) struct SerializableAttributeList<'a>(pub Vec<(&'a str, &'a str)>);

impl<'a> SerializableAttributeList<'a> {
    fn from_resource_vec(rv: &'a AttributeList, ss: &'a StringStore) -> Self {
        let key_values = rv
            .iter()
            .filter_map(|(k, v)| {
                ss.redeem(*k).and_then(|key| {
                    let value = ss.redeem(*v)?;
                    Some((key, value))
                })
            })
            .collect();
        Self(key_values)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum MetricType {
    Counter,
    Gauge,
}

#[derive(Debug, Clone)]
struct Metric {
    name: InternHandle,
    description: InternHandle,
    unit: Option<InternHandle>,
    resources: AttributeList,
    last_fetched_at: jiff::Timestamp,
    datum: MetricDatum,
    metric_type: MetricType,
}

impl Metric {
    /// The ID of a series is the name, combined with the unique set of k/v pairs in the resource
    /// list
    fn stable_id(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        hasher.write_usize(self.name.0);
        hasher.write_usize(self.resources.len());
        for (k, v) in &self.resources {
            hasher.write_usize(k.0);
            hasher.write_usize(v.0);
        }
        hasher.finish()
    }
}

#[derive(Debug)]
pub(crate) struct SerializableMetric<'a> {
    pub label: &'a str,
    pub description: &'a str,
    pub unit: Option<&'a str>,
    pub resources: SerializableAttributeList<'a>,
    pub last_fetched_at: jiff::Timestamp,
    pub datum: MetricDatum,
    pub metric_type: MetricType,
}

impl<'a> SerializableMetric<'a> {
    fn from_metric(m: &'a Metric, ss: &'a StringStore) -> Self {
        Self {
            label: ss.redeem(m.name).unwrap_or_default(),
            description: ss.redeem(m.description).unwrap_or_default(),
            unit: m.unit.as_ref().and_then(|u| ss.redeem(*u)),
            last_fetched_at: m.last_fetched_at,
            resources: SerializableAttributeList::from_resource_vec(&m.resources, ss),
            datum: m.datum,
            metric_type: m.metric_type,
        }
    }
}

#[derive(Default)]
/// Store for the most recent copy of every metric
///
/// Metrics are uniquely identified by their name and the set of their
/// labels. Not all metrics may be seen between every emission, and we don't
/// want to throw away data just because a metric is stale.
///
/// Also, there are a ton of tiny strings here, so we do our best to intern them
/// to reduce allocations.
pub(crate) struct MostRecentMetricStoreInner {
    strings: StringStore,
    metrics: BTreeMap<u64, Metric>,
}

// The structure of a Metric is kind of annoying (the u64/f64/i64 distinction is
// at the top level, then the gauge/counter/histogram distinction under that), so
// I decided to use a visitor instead of an external iterator
struct DataPointVisitor<'a> {
    raw: &'a opentelemetry_sdk::metrics::data::Metric,
    strings: &'a mut StringStore,
    metrics: &'a mut BTreeMap<u64, Metric>,
    scope_attributes: &'a AttributeList,
}

impl<'a> DataPointVisitor<'a> {
    fn visit_point<'b, T, I>(
        &mut self,
        last_fetched_at: jiff::Timestamp,
        value: T,
        point_attributes: I,
        metric_type: MetricType,
    ) where
        T: Into<MetricDatum> + Copy + 'static,
        I: Iterator<Item = &'b opentelemetry::KeyValue>,
    {
        let mut resources = self.scope_attributes.to_owned();
        resources.extend(point_attributes.map(|attribute| {
            let key = self.strings.intern(attribute.key.as_str());
            let value = self.strings.intern(attribute.value.as_str());
            (key, value)
        }));
        let unit = if self.raw.unit().is_empty() {
            None
        } else {
            Some(self.strings.intern(self.raw.unit()))
        };
        let metric = Metric {
            name: self.strings.intern(self.raw.name()),
            description: self.strings.intern(self.raw.description()),
            unit,
            resources,
            last_fetched_at,
            datum: value.into(),
            metric_type,
        };
        let id = metric.stable_id();
        self.metrics.insert(id, metric);
    }

    fn visit_gauge<T>(&mut self, gauge: &opentelemetry_sdk::metrics::data::Gauge<T>)
    where
        T: Into<MetricDatum> + Copy + 'static,
    {
        let Ok(time) = gauge.time().try_into() else {
            tracing::warn!(timestamp = ?gauge.time(), "wildly unusual timestamp in gauge");
            return;
        };
        for data_point in gauge.data_points() {
            self.visit_point(
                time,
                data_point.value(),
                data_point.attributes(),
                MetricType::Gauge,
            );
        }
    }

    fn visit_counter<T>(&mut self, sum: &opentelemetry_sdk::metrics::data::Sum<T>)
    where
        T: Into<MetricDatum> + Copy + 'static,
    {
        let Ok(time) = sum.time().try_into() else {
            tracing::warn!(timestamp = ?sum.time(), "wildly unusual timestamp in counter");
            return;
        };
        for data_point in sum.data_points() {
            self.visit_point(
                time,
                data_point.value(),
                data_point.attributes(),
                MetricType::Counter,
            );
        }
    }

    fn visit_aggregated<T>(&mut self, data: &MetricData<T>)
    where
        T: Into<MetricDatum> + Copy + 'static,
    {
        match data {
            MetricData::Gauge(g) => self.visit_gauge(g),
            MetricData::Sum(c) => self.visit_counter(c),
            // TODO: figure out a good way to show histograms
            MetricData::Histogram(_h) => {}
            MetricData::ExponentialHistogram(_e) => {}
        }
    }

    fn visit_metric(&mut self) {
        match self.raw.data() {
            AggregatedMetrics::F64(f) => self.visit_aggregated(f),
            AggregatedMetrics::U64(u) => self.visit_aggregated(u),
            AggregatedMetrics::I64(i) => self.visit_aggregated(i),
        }
    }
}

impl MostRecentMetricStoreInner {
    fn export(&mut self, metrics: &ResourceMetrics) {
        let resources = metrics
            .resource()
            .iter()
            .filter_map(|(key, value)| {
                let key = key.as_str();
                if key.starts_with("telemetry.") || key == "instance_id" {
                    return None;
                }
                let key = self.strings.intern(key);
                let value = self.strings.intern(value.as_str());
                Some((key, value))
            })
            .collect::<BTreeSet<_>>();
        for item in metrics.scope_metrics() {
            let mut scope_resources = resources.clone();
            scope_resources.extend(item.scope().attributes().filter_map(|attribute| {
                let key = attribute.key.as_str();
                if key.starts_with("telemetry.") || key == "instance_id" {
                    return None;
                }
                let key = self.strings.intern(key);
                let value = self.strings.intern(attribute.value.as_str());
                Some((key, value))
            }));
            for metric in item.metrics() {
                let mut visitor = DataPointVisitor {
                    raw: metric,
                    strings: &mut self.strings,
                    metrics: &mut self.metrics,
                    scope_attributes: &scope_resources,
                };
                visitor.visit_metric();
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct MostRecentMetricStore {
    inner: Arc<Mutex<MostRecentMetricStoreInner>>,
}

impl MostRecentMetricStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Access a shallow copy of the current metrics that has all of the interned
    /// strings reified
    ///
    /// This borrows from the mutex, so can only be accessed through a callback
    pub(crate) fn serialize_with<F, R>(&self, f: F) -> R
    where
        for<'a> F: FnOnce(Vec<SerializableMetric<'a>>) -> R,
    {
        let guard = self.inner.lock();
        let strings = &guard.strings;
        let serializable = guard
            .metrics
            .values()
            .map(move |m| SerializableMetric::from_metric(m, strings))
            .collect();
        f(serializable)
    }
}

impl PushMetricExporter for MostRecentMetricStore {
    fn export(
        &self,
        metrics: &ResourceMetrics,
    ) -> impl Future<Output = opentelemetry_sdk::error::OTelSdkResult> + Send {
        let mut guard = self.inner.lock();
        async move {
            guard.export(metrics);
            Ok(())
        }
    }

    fn temporality(&self) -> opentelemetry_sdk::metrics::Temporality {
        opentelemetry_sdk::metrics::Temporality::Cumulative
    }

    fn shutdown_with_timeout(
        &self,
        _timeout: std::time::Duration,
    ) -> opentelemetry_sdk::error::OTelSdkResult {
        Ok(())
    }

    fn force_flush(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::StringStore;

    #[test]
    fn test_string_store() {
        let mut ss = StringStore::default();

        let handle1 = ss.intern("foo1");
        let handle2 = ss.intern("foo2");
        assert_eq!(handle1, ss.intern("foo1"));
        assert_eq!(handle1, ss.intern(String::from("foo1")));
        assert_eq!(handle1, ss.intern(Cow::Borrowed("foo1")));
        assert_ne!(handle1, handle2);
        assert_eq!(ss.redeem(handle1), Some("foo1"));
        assert_eq!(ss.redeem(handle2), Some("foo2"));
    }
}
