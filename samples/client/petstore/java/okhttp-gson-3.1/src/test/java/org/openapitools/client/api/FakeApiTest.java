/*
 * OpenAPI Petstore
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package org.openapitools.client.api;

import org.openapitools.client.ApiException;
import org.openapitools.client.model.CodesEnum;
import org.openapitools.client.model.RefRefToPathLevelParameterOneofRefToOneofParameter;
import org.openapitools.client.model.RefToRefParameterAnyofRefToAnyofParameter;
import java.util.UUID;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * API tests for FakeApi
 */
@Disabled
public class FakeApiTest {

    private final FakeApi api = new FakeApi();

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void fakeInlineSchemaAnyofPath1GetTest() throws ApiException {
        Object response = api.fakeInlineSchemaAnyofPath1Get();
        // TODO: test validations
    }

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void fakeInlineSchemaAnyofPath2GetTest() throws ApiException {
        Object response = api.fakeInlineSchemaAnyofPath2Get();
        // TODO: test validations
    }

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void fakeInlineSchemaAnyofPath3GetTest() throws ApiException {
        List<Object> response = api.fakeInlineSchemaAnyofPath3Get();
        // TODO: test validations
    }

    /**
     * op1
     *
     * @throws ApiException if the Api call fails
     */
    @Test
    public void op1Test() throws ApiException {
        Object response = api.op1();
        // TODO: test validations
    }

    /**
     * op2
     *
     * @throws ApiException if the Api call fails
     */
    @Test
    public void op2Test() throws ApiException {
        Object response = api.op2();
        // TODO: test validations
    }

    /**
     * op3
     *
     * @throws ApiException if the Api call fails
     */
    @Test
    public void op3Test() throws ApiException {
        List<CodesEnum> queryEnum = null;
        api.op3(queryEnum);
        // TODO: test validations
    }

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void refToRefParameterTest() throws ApiException {
        UUID refToUuid = null;
        String response = api.refToRefParameter(refToUuid);
        // TODO: test validations
    }

    /**
     * to test $ref to operation level parameters
     *
     * @throws ApiException if the Api call fails
     */
    @Test
    public void refToRefParameterAnyofTest() throws ApiException {
        RefToRefParameterAnyofRefToAnyofParameter refToAnyof = null;
        api.refToRefParameterAnyof(refToAnyof);
        // TODO: test validations
    }

    /**
     * to test $ref to path level parameters
     *
     * @throws ApiException if the Api call fails
     */
    @Test
    public void refToRefParameterOneofTest() throws ApiException {
        RefRefToPathLevelParameterOneofRefToOneofParameter refToOneof = null;
        api.refToRefParameterOneof(refToOneof);
        // TODO: test validations
    }

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void responseNoRefTest() throws ApiException {
        String response = api.responseNoRef();
        // TODO: test validations
    }

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void responseRefToNoRefTest() throws ApiException {
        String response = api.responseRefToNoRef();
        // TODO: test validations
    }

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void responseRefToRefTest() throws ApiException {
        String response = api.responseRefToRef();
        // TODO: test validations
    }

}
