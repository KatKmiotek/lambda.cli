import { APIGatewayEvent, Context, APIGatewayProxyCallback } from 'aws-lambda';
import { handler } from '../index';
import { expect, test, describe, beforeEach, afterEach, vitest } from 'vitest'


describe('Lambda Handler Tests', () => {
    let mockContext: Context;
    let mockCallback: APIGatewayProxyCallback;
    let mockEvent: APIGatewayEvent;

    beforeEach(() => {
        vitest.spyOn(console, 'log').mockImplementation(() => {});

        mockContext = {
            callbackWaitsForEmptyEventLoop: true,
            functionName: 'test-function',
            functionVersion: '1',
            invokedFunctionArn: 'test-arn',
            memoryLimitInMB: '128',
            awsRequestId: 'test-request-id',
            logGroupName: 'test-log-group',
            logStreamName: 'test-log-stream',
            getRemainingTimeInMillis: () => 1000,
            done: () => {},
            fail: () => {},
            succeed: () => {},
        };

        mockEvent = {
            body: null,
            headers: {},
            multiValueHeaders: {},
            httpMethod: 'GET',
            isBase64Encoded: false,
            path: '/test',
            pathParameters: null,
            queryStringParameters: null,
            multiValueQueryStringParameters: null,
            stageVariables: null,
            requestContext: {
                accountId: '',
                apiId: '',
                authorizer: {},
                protocol: '',
                httpMethod: 'GET',
                identity: {
                    accessKey: null,
                    accountId: null,
                    apiKey: null,
                    apiKeyId: null,
                    caller: null,
                    clientCert: null,
                    cognitoAuthenticationProvider: null,
                    cognitoAuthenticationType: null,
                    cognitoIdentityId: null,
                    cognitoIdentityPoolId: null,
                    principalOrgId: null,
                    sourceIp: '',
                    user: null,
                    userAgent: null,
                    userArn: null,
                },
                path: '/test',
                stage: 'test',
                requestId: '',
                requestTimeEpoch: 0,
                resourceId: '',
                resourcePath: '',
            },
            resource: '',
        };

        mockCallback = vitest.fn();
    });

    afterEach(() => {
        vitest.clearAllMocks();
    });

    test('should return 200 status code', () => {
        handler(mockEvent, mockContext, mockCallback);

        expect(mockCallback).toHaveBeenCalledWith(
            null,
            expect.objectContaining({
                statusCode: 200,
            })
        );
    });

    test('should return expected message in response body', () => {
        handler(mockEvent, mockContext, mockCallback);

        expect(mockCallback).toHaveBeenCalledWith(
            null,
            expect.objectContaining({
                body: expect.stringContaining('Hello from'),
            })
        );
    });

    test('should log event and context', () => {
        handler(mockEvent, mockContext, mockCallback);

        expect(console.log).toHaveBeenCalledWith(
            `Event: ${JSON.stringify(mockEvent, null, 2)}`
        );
        expect(console.log).toHaveBeenCalledWith(
            `Context: ${JSON.stringify(mockContext, null, 2)}`
        );
    });
});
