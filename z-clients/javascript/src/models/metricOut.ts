// this file is @generated
import {
    type MetricType,
    MetricTypeSerializer,
} from './metricType';

export interface MetricOut {
    /** Label for this series */
    label: string;
    /** Human-readable description of this series */
    description: string;
    /** Key/Value pairs attached to this sequence */
    attributes: { [key: string]: string };
    /**
     * Most recent data point for this series
     * 
     * All points (u64, i64, and f64) are squished into an f64, be careful
     * of inexactness for values above 2**53.
     */
    value: number;
    /**
     * Type of this metric
     * 
     * Histograms are not currently exported through this API, and can
     * only be accessed through OTLP.
     */
    metricType: MetricType;
    /** Timestamp this metric was collected */
    timestamp: Date;
    /**
     * Optional unit, following UCUM unit conventions if possible
     * 
     * See https://ucum.org/ for details
     */
    unit?: string | null;
}

export const MetricOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MetricOut {
        return {
            label: object['label'],
            description: object['description'],
            attributes: object['attributes'],
            value: object['value'],
            metricType: MetricTypeSerializer._fromJsonObject(object['metric_type']),
            timestamp: new Date(Number(object['timestamp'])),
            unit: object['unit'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MetricOut): any {
        return {
            'label': self.label,
            'description': self.description,
            'attributes': self.attributes,
            'value': self.value,
            'metric_type': MetricTypeSerializer._toJsonObject(self.metricType),
            'timestamp': self.timestamp.getTime(),
            'unit': self.unit,
        };
    }
}