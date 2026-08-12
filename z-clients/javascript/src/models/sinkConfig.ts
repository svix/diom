// this file is @generated
import {
    type HttpSinkConfig,
    HttpSinkConfigSerializer,
} from './httpSinkConfig';
import {
    type KafkaSinkConfig,
    KafkaSinkConfigSerializer,
} from './kafkaSinkConfig';
import {
    type SvixSinkConfig,
    SvixSinkConfigSerializer,
} from './svixSinkConfig';


// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface _SinkConfigFields {}







interface SinkConfigHttp {
    type: 'http';
    data: HttpSinkConfig;
    
}

interface SinkConfigSvix {
    type: 'svix';
    data: SvixSinkConfig;
    
}

interface SinkConfigKafka {
    type: 'kafka';
    data: KafkaSinkConfig;
    
}



export type SinkConfig = _SinkConfigFields & (| SinkConfigHttp
    | SinkConfigSvix
    | SinkConfigKafka
    );

export const SinkConfigSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SinkConfig {
        const type = object['type'];

        // biome-ignore lint/suspicious/noExplicitAny: intentional any
        function getData(type: string): any {
            switch (type) {
                case 'http':
                    return HttpSinkConfigSerializer._fromJsonObject(
                            object['data']
                        );
                case 'svix':
                    return SvixSinkConfigSerializer._fromJsonObject(
                            object['data']
                        );
                case 'kafka':
                    return KafkaSinkConfigSerializer._fromJsonObject(
                            object['data']
                        );default:
                    throw new Error(`Unexpected type: ${ type }`);
            }
        }

        return {
            type,
            data:getData(type),
            };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SinkConfig): any {
        // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
        let data;
        switch (self.type) {
            case 'http':
                data =
                    HttpSinkConfigSerializer._toJsonObject(
                        self.data
                    );
                break;
            case 'svix':
                data =
                    SvixSinkConfigSerializer._toJsonObject(
                        self.data
                    );
                break;
            case 'kafka':
                data =
                    KafkaSinkConfigSerializer._toJsonObject(
                        self.data
                    );
                break;}

        return {
            'type': self.type,
            'data': data,
            };
    }
}