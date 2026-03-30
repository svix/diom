// this file is @generated
import {
    type AccessRuleEffect,
    AccessRuleEffectSerializer,
} from './accessRuleEffect';

export interface AccessRule {
    effect: AccessRuleEffect;
    resource: string;
    actions: string[];
}

export const AccessRuleSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AccessRule {
        return {
            effect: AccessRuleEffectSerializer._fromJsonObject(object['effect']),
            resource: object['resource'],
            actions: object['actions'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AccessRule): any {
        return {
            'effect': AccessRuleEffectSerializer._toJsonObject(self.effect),
            'resource': self.resource,
            'actions': self.actions,
        };
    }
}