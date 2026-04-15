// this file is @generated

export interface AdminAccessPolicyConfigureOut {
    id: string;
    created: Date;
    updated: Date;
}

export const AdminAccessPolicyConfigureOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyConfigureOut {
        return {
            id: object['id'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyConfigureOut): any {
        return {
            'id': self.id,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}