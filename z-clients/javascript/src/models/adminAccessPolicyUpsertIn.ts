// this file is @generated
import {
    type AccessRule,
    AccessRuleSerializer,
} from './accessRule';

export interface AdminAccessPolicyUpsertIn {
    id: string;
    description: string;
    rules?: AccessRule[];
}

export const AdminAccessPolicyUpsertInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyUpsertIn {
        return {
            id: object['id'],
            description: object['description'],
            rules: object['rules']?.map((item: AccessRule) => AccessRuleSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyUpsertIn): any {
        return {
            'id': self.id,
            'description': self.description,
            'rules': self.rules?.map((item) => AccessRuleSerializer._toJsonObject(item)),
        };
    }
}