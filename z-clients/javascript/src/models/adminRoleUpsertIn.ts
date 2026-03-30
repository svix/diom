// this file is @generated
import {
    type AccessRule,
    AccessRuleSerializer,
} from './accessRule';

export interface AdminRoleUpsertIn {
    id: string;
    description: string;
    rules?: AccessRule[];
    policies?: string[];
    context?: { [key: string]: string };
}

export const AdminRoleUpsertInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleUpsertIn {
        return {
            id: object['id'],
            description: object['description'],
            rules: object['rules']?.map((item: AccessRule) => AccessRuleSerializer._fromJsonObject(item)),
            policies: object['policies'],
            context: object['context'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleUpsertIn): any {
        return {
            'id': self.id,
            'description': self.description,
            'rules': self.rules?.map((item) => AccessRuleSerializer._toJsonObject(item)),
            'policies': self.policies,
            'context': self.context,
        };
    }
}