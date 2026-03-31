// this file is @generated

export enum AccessRuleEffect {
    Allow = 'allow',
    Deny = 'deny',
    }

export const AccessRuleEffectSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AccessRuleEffect {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AccessRuleEffect): any {
        return self;
    }
}