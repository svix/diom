// this file is @generated
import {
    type AccessRule,
    AccessRuleSerializer,
} from './accessRule';

export interface AdminAccessPolicyOut {
    id: string;
    description: string;
    rules: AccessRule[];
    created: Date;
    updated: Date;
}

export const AdminAccessPolicyOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyOut {
        return {
            id: object['id'],
            description: object['description'],
            rules: object['rules'].map((item: AccessRule) => AccessRuleSerializer._fromJsonObject(item)),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyOut): any {
        return {
            'id': self.id,
            'description': self.description,
            'rules': self.rules.map((item) => AccessRuleSerializer._toJsonObject(item)),
            'created': self.created,
            'updated': self.updated,
        };
    }
}