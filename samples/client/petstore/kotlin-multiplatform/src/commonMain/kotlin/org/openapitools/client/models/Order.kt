/**
 *
 * Please note:
 * This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit this file manually.
 *
 */

@file:Suppress(
    "ArrayInDataClass",
    "EnumEntryName",
    "RemoveRedundantQualifierName",
    "UnusedImport"
)

package org.openapitools.client.models


import kotlinx.serialization.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

/**
 * An order for a pets from the pet store
 *
 * @param id 
 * @param petId 
 * @param quantity 
 * @param shipDate 
 * @param status Order Status
 * @param complete 
 */
@Serializable

data class Order (

    @SerialName(value = "id") val id: kotlin.Long? = null,

    @SerialName(value = "petId") val petId: kotlin.Long? = null,

    @SerialName(value = "quantity") val quantity: kotlin.Int? = null,

    @SerialName(value = "shipDate") val shipDate: kotlin.String? = null,

    /* Order Status */
    @SerialName(value = "status") val status: Order.Status? = null,

    @SerialName(value = "complete") val complete: kotlin.Boolean? = false

) {

    /**
     * Order Status
     *
     * Values: placed,approved,delivered
     */
    @Serializable
    enum class Status(val value: kotlin.String) {
        @SerialName(value = "placed") placed("placed"),
        @SerialName(value = "approved") approved("approved"),
        @SerialName(value = "delivered") delivered("delivered");
    }
}

