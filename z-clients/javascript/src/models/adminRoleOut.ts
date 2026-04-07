// this file is @generated
import {
    type AccessRule,
    AccessRuleSerializer,
} from './accessRule';

export interface AdminRoleOut {
    id: string;
    description: string;
    rules: AccessRule[];
    policies: string[];
    context: { [key: string]: string };
    created: number;
    updated: number;
}

export const AdminRoleOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleOut {
        return {
            id: object['id'],
            description: object['description'],
            rules: object['rules'].map((item: AccessRule) => AccessRuleSerializer._fromJsonObject(item)),
            policies: object['policies'],
            context: object['context'],
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleOut): any {
        return {
            'id': self.id,
            'description': self.description,
            'rules': self.rules.map((item) => AccessRuleSerializer._toJsonObject(item)),
            'policies': self.policies,
            'context': self.context,
            'created': self.created,
            'updated': self.updated,
        };
    }
}