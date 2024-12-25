// <auto-generated>
/*
 * OpenAPI Petstore
 *
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 *
 * The version of the OpenAPI document: 1.0.0
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */

using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.ComponentModel.DataAnnotations;
using OpenAPIClientUtils = Org.OpenAPITools.Client.ClientUtils;
using Org.OpenAPITools.Client;

namespace Org.OpenAPITools.Model
{
    /// <summary>
    /// Mixed oneOf types for testing
    /// </summary>
    public partial class MixedOneOfContent : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="MixedOneOfContent" /> class.
        /// </summary>
        /// <param name="string"></param>
        public MixedOneOfContent(string @string)
        {
            String = @string;
            OnCreated();
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="MixedOneOfContent" /> class.
        /// </summary>
        /// <param name="bool"></param>
        public MixedOneOfContent(bool @bool)
        {
            Bool = @bool;
            OnCreated();
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="MixedOneOfContent" /> class.
        /// </summary>
        /// <param name="int"></param>
        public MixedOneOfContent(int @int)
        {
            Int = @int;
            OnCreated();
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="MixedOneOfContent" /> class.
        /// </summary>
        /// <param name="decimal"></param>
        public MixedOneOfContent(decimal @decimal)
        {
            Decimal = @decimal;
            OnCreated();
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="MixedOneOfContent" /> class.
        /// </summary>
        /// <param name="mixedSubId"></param>
        public MixedOneOfContent(MixedSubId mixedSubId)
        {
            MixedSubId = mixedSubId;
            OnCreated();
        }

        partial void OnCreated();

        /// <summary>
        /// Gets or Sets String
        /// </summary>
        public string String { get; set; }

        /// <summary>
        /// Gets or Sets Bool
        /// </summary>
        public bool? Bool { get; set; }

        /// <summary>
        /// Gets or Sets Int
        /// </summary>
        public int? Int { get; set; }

        /// <summary>
        /// Gets or Sets Decimal
        /// </summary>
        public decimal? Decimal { get; set; }

        /// <summary>
        /// Gets or Sets MixedSubId
        /// </summary>
        public MixedSubId MixedSubId { get; set; }

        /// <summary>
        /// Gets or Sets additional properties
        /// </summary>
        [JsonExtensionData]
        public Dictionary<string, JsonElement> AdditionalProperties { get; } = new Dictionary<string, JsonElement>();

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class MixedOneOfContent {\n");
            sb.Append("  AdditionalProperties: ").Append(AdditionalProperties).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            yield break;
        }
    }

    /// <summary>
    /// A Json converter for type <see cref="MixedOneOfContent" />
    /// </summary>
    public class MixedOneOfContentJsonConverter : JsonConverter<MixedOneOfContent>
    {
        /// <summary>
        /// Deserializes json to <see cref="MixedOneOfContent" />
        /// </summary>
        /// <param name="utf8JsonReader"></param>
        /// <param name="typeToConvert"></param>
        /// <param name="jsonSerializerOptions"></param>
        /// <returns></returns>
        /// <exception cref="JsonException"></exception>
        public override MixedOneOfContent Read(ref Utf8JsonReader utf8JsonReader, Type typeToConvert, JsonSerializerOptions jsonSerializerOptions)
        {
            int currentDepth = utf8JsonReader.CurrentDepth;

            if (utf8JsonReader.TokenType != JsonTokenType.StartObject && utf8JsonReader.TokenType != JsonTokenType.StartArray)
                throw new JsonException();

            JsonTokenType startingTokenType = utf8JsonReader.TokenType;

            string varString = default;
            bool? varBool = default;
            int? varInt = default;
            decimal? varDecimal = default;
            MixedSubId mixedSubId = default;

            Utf8JsonReader utf8JsonReaderOneOf = utf8JsonReader;
            while (utf8JsonReaderOneOf.Read())
            {
                if (startingTokenType == JsonTokenType.StartObject && utf8JsonReaderOneOf.TokenType == JsonTokenType.EndObject && currentDepth == utf8JsonReaderOneOf.CurrentDepth)
                    break;

                if (startingTokenType == JsonTokenType.StartArray && utf8JsonReaderOneOf.TokenType == JsonTokenType.EndArray && currentDepth == utf8JsonReaderOneOf.CurrentDepth)
                    break;

                if (utf8JsonReaderOneOf.TokenType == JsonTokenType.PropertyName && currentDepth == utf8JsonReaderOneOf.CurrentDepth - 1)
                {
                    Utf8JsonReader utf8JsonReaderString = utf8JsonReader;
                    ClientUtils.TryDeserialize<string>(ref utf8JsonReaderString, jsonSerializerOptions, out varString);

                    Utf8JsonReader utf8JsonReaderBool = utf8JsonReader;
                    ClientUtils.TryDeserialize<bool?>(ref utf8JsonReaderBool, jsonSerializerOptions, out varBool);

                    Utf8JsonReader utf8JsonReaderInt = utf8JsonReader;
                    ClientUtils.TryDeserialize<int?>(ref utf8JsonReaderInt, jsonSerializerOptions, out varInt);

                    Utf8JsonReader utf8JsonReaderDecimal = utf8JsonReader;
                    ClientUtils.TryDeserialize<decimal?>(ref utf8JsonReaderDecimal, jsonSerializerOptions, out varDecimal);

                    Utf8JsonReader utf8JsonReaderMixedSubId = utf8JsonReader;
                    ClientUtils.TryDeserialize<MixedSubId>(ref utf8JsonReaderMixedSubId, jsonSerializerOptions, out mixedSubId);
                }
            }

            while (utf8JsonReader.Read())
            {
                if (startingTokenType == JsonTokenType.StartObject && utf8JsonReader.TokenType == JsonTokenType.EndObject && currentDepth == utf8JsonReader.CurrentDepth)
                    break;

                if (startingTokenType == JsonTokenType.StartArray && utf8JsonReader.TokenType == JsonTokenType.EndArray && currentDepth == utf8JsonReader.CurrentDepth)
                    break;

                if (utf8JsonReader.TokenType == JsonTokenType.PropertyName && currentDepth == utf8JsonReader.CurrentDepth - 1)
                {
                    string localVarJsonPropertyName = utf8JsonReader.GetString();
                    utf8JsonReader.Read();

                    switch (localVarJsonPropertyName)
                    {
                        default:
                            break;
                    }
                }
            }

            if (varString != null)
                return new MixedOneOfContent(varString);

            if (varBool != null)
                return new MixedOneOfContent(varBool.Value);

            if (varInt != null)
                return new MixedOneOfContent(varInt.Value);

            if (varDecimal != null)
                return new MixedOneOfContent(varDecimal.Value);

            if (mixedSubId != null)
                return new MixedOneOfContent(mixedSubId);

            throw new JsonException();
        }

        /// <summary>
        /// Serializes a <see cref="MixedOneOfContent" />
        /// </summary>
        /// <param name="writer"></param>
        /// <param name="mixedOneOfContent"></param>
        /// <param name="jsonSerializerOptions"></param>
        /// <exception cref="NotImplementedException"></exception>
        public override void Write(Utf8JsonWriter writer, MixedOneOfContent mixedOneOfContent, JsonSerializerOptions jsonSerializerOptions)
        {
            writer.WriteStartObject();

            WriteProperties(writer, mixedOneOfContent, jsonSerializerOptions);
            writer.WriteEndObject();
        }

        /// <summary>
        /// Serializes the properties of <see cref="MixedOneOfContent" />
        /// </summary>
        /// <param name="writer"></param>
        /// <param name="mixedOneOfContent"></param>
        /// <param name="jsonSerializerOptions"></param>
        /// <exception cref="NotImplementedException"></exception>
        public void WriteProperties(Utf8JsonWriter writer, MixedOneOfContent mixedOneOfContent, JsonSerializerOptions jsonSerializerOptions)
        {

        }
    }
}
