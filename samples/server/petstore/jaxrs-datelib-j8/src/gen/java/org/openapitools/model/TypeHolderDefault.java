/*
 * OpenAPI Petstore
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package org.openapitools.model;

import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;
import java.io.Serializable;
import javax.validation.constraints.*;
import javax.validation.Valid;

/**
 * TypeHolderDefault
 */
@JsonPropertyOrder({
  TypeHolderDefault.JSON_PROPERTY_STRING_ITEM,
  TypeHolderDefault.JSON_PROPERTY_NUMBER_ITEM,
  TypeHolderDefault.JSON_PROPERTY_INTEGER_ITEM,
  TypeHolderDefault.JSON_PROPERTY_BOOL_ITEM,
  TypeHolderDefault.JSON_PROPERTY_ARRAY_ITEM
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaJerseyServerCodegen", comments = "Generator version: 7.11.0-SNAPSHOT")
public class TypeHolderDefault  implements Serializable {
  public static final String JSON_PROPERTY_STRING_ITEM = "string_item";
  @JsonProperty(JSON_PROPERTY_STRING_ITEM)
  private String stringItem = "what";

  public static final String JSON_PROPERTY_NUMBER_ITEM = "number_item";
  @JsonProperty(JSON_PROPERTY_NUMBER_ITEM)
  private BigDecimal numberItem;

  public static final String JSON_PROPERTY_INTEGER_ITEM = "integer_item";
  @JsonProperty(JSON_PROPERTY_INTEGER_ITEM)
  private Integer integerItem;

  public static final String JSON_PROPERTY_BOOL_ITEM = "bool_item";
  @JsonProperty(JSON_PROPERTY_BOOL_ITEM)
  private Boolean boolItem = true;

  public static final String JSON_PROPERTY_ARRAY_ITEM = "array_item";
  @JsonProperty(JSON_PROPERTY_ARRAY_ITEM)
  private List<Integer> arrayItem = new ArrayList<>();

  public TypeHolderDefault stringItem(String stringItem) {
    this.stringItem = stringItem;
    return this;
  }

  /**
   * Get stringItem
   * @return stringItem
   **/
  @JsonProperty(value = "string_item")
  @ApiModelProperty(required = true, value = "")
  @NotNull 
  public String getStringItem() {
    return stringItem;
  }

  public void setStringItem(String stringItem) {
    this.stringItem = stringItem;
  }

  public TypeHolderDefault numberItem(BigDecimal numberItem) {
    this.numberItem = numberItem;
    return this;
  }

  /**
   * Get numberItem
   * @return numberItem
   **/
  @JsonProperty(value = "number_item")
  @ApiModelProperty(required = true, value = "")
  @NotNull @Valid 
  public BigDecimal getNumberItem() {
    return numberItem;
  }

  public void setNumberItem(BigDecimal numberItem) {
    this.numberItem = numberItem;
  }

  public TypeHolderDefault integerItem(Integer integerItem) {
    this.integerItem = integerItem;
    return this;
  }

  /**
   * Get integerItem
   * @return integerItem
   **/
  @JsonProperty(value = "integer_item")
  @ApiModelProperty(required = true, value = "")
  @NotNull 
  public Integer getIntegerItem() {
    return integerItem;
  }

  public void setIntegerItem(Integer integerItem) {
    this.integerItem = integerItem;
  }

  public TypeHolderDefault boolItem(Boolean boolItem) {
    this.boolItem = boolItem;
    return this;
  }

  /**
   * Get boolItem
   * @return boolItem
   **/
  @JsonProperty(value = "bool_item")
  @ApiModelProperty(required = true, value = "")
  @NotNull 
  public Boolean getBoolItem() {
    return boolItem;
  }

  public void setBoolItem(Boolean boolItem) {
    this.boolItem = boolItem;
  }

  public TypeHolderDefault arrayItem(List<Integer> arrayItem) {
    this.arrayItem = arrayItem;
    return this;
  }

  public TypeHolderDefault addArrayItemItem(Integer arrayItemItem) {
    if (this.arrayItem == null) {
      this.arrayItem = new ArrayList<>();
    }
    this.arrayItem.add(arrayItemItem);
    return this;
  }

  /**
   * Get arrayItem
   * @return arrayItem
   **/
  @JsonProperty(value = "array_item")
  @ApiModelProperty(required = true, value = "")
  @NotNull 
  public List<Integer> getArrayItem() {
    return arrayItem;
  }

  public void setArrayItem(List<Integer> arrayItem) {
    this.arrayItem = arrayItem;
  }


  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    TypeHolderDefault typeHolderDefault = (TypeHolderDefault) o;
    return Objects.equals(this.stringItem, typeHolderDefault.stringItem) &&
        Objects.equals(this.numberItem, typeHolderDefault.numberItem) &&
        Objects.equals(this.integerItem, typeHolderDefault.integerItem) &&
        Objects.equals(this.boolItem, typeHolderDefault.boolItem) &&
        Objects.equals(this.arrayItem, typeHolderDefault.arrayItem);
  }

  @Override
  public int hashCode() {
    return Objects.hash(stringItem, numberItem, integerItem, boolItem, arrayItem);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class TypeHolderDefault {\n");
    
    sb.append("    stringItem: ").append(toIndentedString(stringItem)).append("\n");
    sb.append("    numberItem: ").append(toIndentedString(numberItem)).append("\n");
    sb.append("    integerItem: ").append(toIndentedString(integerItem)).append("\n");
    sb.append("    boolItem: ").append(toIndentedString(boolItem)).append("\n");
    sb.append("    arrayItem: ").append(toIndentedString(arrayItem)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }
}

