package com.svix.coyote;

import java.io.IOException;
import java.net.InetAddress;
import java.net.Socket;
import java.net.SocketException;
import java.net.UnknownHostException;
import java.util.Arrays;
import java.util.concurrent.locks.LockSupport;
import java.util.concurrent.ThreadLocalRandom;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import javax.net.SocketFactory;

import com.fasterxml.jackson.databind.ObjectMapper;

import okhttp3.Headers;
import okhttp3.HttpUrl;
import okhttp3.MediaType;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.RequestBody;
import okhttp3.Response;
import okhttp3.Protocol;

class HelpImTrappedInANagleFactory extends SocketFactory {
    public Socket createSocket()
        throws SocketException
    {
        Socket socket = new Socket();
        socket.setTcpNoDelay(true);
        return socket;
    }

    public Socket createSocket(String host, int port)
    throws IOException, UnknownHostException, SocketException
    {
        Socket socket = new Socket(host, port);
        socket.setTcpNoDelay(true);
        return socket;
    }

    public Socket createSocket(InetAddress address, int port)
    throws IOException, SocketException
    {
        Socket socket = new Socket(address, port);
        socket.setTcpNoDelay(true);
        return socket;
    }

    public Socket createSocket(String host, int port,
        InetAddress clientAddress, int clientPort)
    throws IOException, UnknownHostException, SocketException
    {
        Socket socket = new Socket(host, port, clientAddress, clientPort);
        socket.setTcpNoDelay(true);
        return socket;
    }

    public Socket createSocket(InetAddress address, int port,
        InetAddress clientAddress, int clientPort)
    throws IOException, SocketException
    {
        Socket socket = new Socket(address, port, clientAddress, clientPort);
        socket.setTcpNoDelay(true);
        return socket;
    }
}

public class HttpClient {
    private final HttpUrl baseUrl;
    private final Map<String, String> defaultHeaders;
    private final List<Long> retrySchedule;
    private final OkHttpClient client;
    private final ObjectMapper objectMapper;

    public HttpClient(
            HttpUrl baseUrl, Map<String, String> defaultHeaders, List<Long> retrySchedule) {
        this.baseUrl = baseUrl;
        this.defaultHeaders = defaultHeaders;
        this.retrySchedule = retrySchedule;
        this.client = new OkHttpClient.Builder()
            .protocols(Arrays.asList(Protocol.H2_PRIOR_KNOWLEDGE, Protocol.HTTP_2, Protocol.HTTP_1_1))
            .socketFactory(new HelpImTrappedInANagleFactory())
            .build();

        this.objectMapper = Utils.getObjectMapper();
    }

    public HttpUrl.Builder newUrlBuilder() {
        return new HttpUrl.Builder()
                .scheme(baseUrl.scheme())
                .host(baseUrl.host())
                .port(baseUrl.port());
    }

    public <Req, Res> Res executeRequest(
            String method, HttpUrl url, Headers headers, Req reqBody, Class<Res> responseClass)
            throws ApiException, IOException {
        Request.Builder reqBuilder = new Request.Builder().url(url);

        // Handle request body
        if (reqBody != null) {
            String jsonBody = objectMapper.writeValueAsString(reqBody);
            RequestBody body = RequestBody.create(jsonBody, MediaType.parse("application/json"));
            reqBuilder.method(method, body);
        } else {
            reqBuilder.method(method, null);
        }

        // Add default headers
        defaultHeaders.forEach(reqBuilder::addHeader);

        String idempotencyKey = headers == null ? null : headers.get("idempotency-key");
        if ((idempotencyKey == null || idempotencyKey.isEmpty()) && method.toUpperCase() == "POST") {
                reqBuilder.addHeader("idempotency-key", "auto_" + UUID.randomUUID().toString());
        }

        // Add custom headers if present
        if (headers != null) {
            headers.forEach(pair -> reqBuilder.addHeader(pair.getFirst(), pair.getSecond()));
        }

        reqBuilder.addHeader(
                "svix-req-id",
                String.valueOf(ThreadLocalRandom.current().nextLong(0, Long.MAX_VALUE)));

        Request request = reqBuilder.build();
        Response response = executeRequestWithRetry(request);

        if (response.body() == null) {
            throw new ApiException("Body is null", response.code(), "");
        }

        String bodyString = response.body().string();

        if (response.code() == 204) {
            return null;
        }

        if (response.code() >= 200 && response.code() < 300) {
            return objectMapper.readValue(bodyString, responseClass);
        }

        throw new ApiException(
                "Non 200 status code: `" + response.code() + "`", response.code(), bodyString);
    }

    private Response executeRequestWithRetry(Request request) throws IOException {
        Response response = client.newCall(request).execute();

        int retryCount = 0;
        while (response.code() >= 500 && retryCount < retrySchedule.size()) {
            response.close();

            // Use LockSupport for precise parking instead of Thread.sleep
            LockSupport.parkNanos(retrySchedule.get(retryCount) * 1_000_000); // Convert ms to ns

            Request retryRequest =
                    request.newBuilder()
                            .header("svix-retry-count", String.valueOf(retryCount + 1))
                            .build();
            response = client.newCall(retryRequest).execute();
            retryCount++;
        }
        return response;
    }
}
