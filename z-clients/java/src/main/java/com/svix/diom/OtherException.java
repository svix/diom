package com.svix.diom;

import lombok.Getter;

import com.svix.diom.DiomException;

@Getter
public class OtherException extends DiomException {
    OtherException(final String operationId, final Exception cause) {
        super(operationId, cause);
    }

    OtherException(final String operationId, final String message) {
        super(operationId, new RuntimeException(message));
    }
}
