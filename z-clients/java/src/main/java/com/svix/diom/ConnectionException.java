package com.svix.diom;

import lombok.Getter;

import com.svix.diom.DiomException;

@Getter
public class ConnectionException extends DiomException {
    ConnectionException(final String operationId, final Exception cause) {
        super(operationId, cause);
    }
}
