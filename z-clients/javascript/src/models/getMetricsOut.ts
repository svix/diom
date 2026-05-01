// this file is @generated
import {
    type MetricOut,
    MetricOutSerializer,
} from './metricOut';

export interface GetMetricsOut {
    metrics: MetricOut[];
}

export const GetMetricsOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): GetMetricsOut {
        return {
            metrics: object['metrics'].map((item: MetricOut) => MetricOutSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: GetMetricsOut): any {
        return {
            'metrics': self.metrics.map((item) => MetricOutSerializer._toJsonObject(item)),
        };
    }
}