package com.svix.diom;

import lombok.Getter;

import com.svix.diom.DiomException;

@Getter
public class OperationErrorException extends DiomException {
    String code;
    String detail;
    String location;

    OperationErrorException(
        final String operationId,
        final String code,
        final String detail,
        final String location
    ) {
        super(operationId, code, detail, location);
        this.code = code;
        this.detail = detail;
        this.location = location;
    }
}
