// this file is @generated

export enum MetricType {
    Counter = 'counter',
    Gauge = 'gauge',
    }

export const MetricTypeSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MetricType {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MetricType): any {
        return self;
    }
}