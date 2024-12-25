//
// SpecialModelName.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation

@available(*, deprecated, renamed: "PetstoreClientAPI.SpecialModelName")
public typealias SpecialModelName = PetstoreClientAPI.SpecialModelName

extension PetstoreClientAPI {

public final class SpecialModelName: @unchecked Sendable, Codable, JSONEncodable, Hashable {

    public private(set) var specialPropertyName: Int64?

    public init(specialPropertyName: Int64? = nil) {
        self.specialPropertyName = specialPropertyName
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case specialPropertyName = "$special[property.name]"
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encodeIfPresent(specialPropertyName, forKey: .specialPropertyName)
    }

    public static func == (lhs: SpecialModelName, rhs: SpecialModelName) -> Bool {
        lhs.specialPropertyName == rhs.specialPropertyName
        
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(specialPropertyName?.hashValue)
        
    }
}

}
