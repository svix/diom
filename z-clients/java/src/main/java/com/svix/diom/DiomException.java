package com.svix.diom;

import lombok.Getter;

@Getter
public class DiomException extends Exception {
    String operationId;

    DiomException(
        final String operationId,
        final String code,
        final String detail,
        final String location
    ) {
        super(DiomException.errorResponseMessage(code, detail, location));
        this.operationId = operationId;
    }

    DiomException(final String operationId, Exception cause) {
        super(cause);
        this.operationId = operationId;
    }

    private static String errorResponseMessage(
        final String code,
        final String detail,
        final String location
    ) {
        StringBuilder b = new StringBuilder("code=");
        b.append(code);
        if (location != null) {
            b.append(" location=");
            b.append(location);
        }
        b.append(" detail=\"");
        b.append(detail.replace("\"", "\\\""));
        b.append("\"");
        return b.toString();
    }
}
