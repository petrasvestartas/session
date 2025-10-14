#pragma once
#include "json.h"
#include <fstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <memory>

namespace session_cpp {
namespace encoders {

/**
 * @brief Write JSON data to file.
 * @param j JSON object to write
 * @param filepath Path to output file
 * @param pretty If true, format with indentation
 */
inline void json_dump(const nlohmann::ordered_json &j,
                      const std::string &filepath,
                      bool pretty = true) {
  std::ofstream ofs(filepath);
  if (!ofs.is_open()) {
    throw std::runtime_error("Failed to open file for writing: " + filepath);
  }
  ofs << (pretty ? j.dump(2) : j.dump());
}

/**
 * @brief Read JSON data from file.
 * @param filepath Path to input file
 * @return JSON object
 */
inline nlohmann::ordered_json json_load_data(const std::string &filepath) {
  std::ifstream ifs(filepath);
  if (!ifs.is_open()) {
    throw std::runtime_error("Failed to open file for reading: " + filepath);
  }
  nlohmann::ordered_json j;
  ifs >> j;
  return j;
}

/**
 * @brief Serialize object to JSON string.
 * @param obj Object with jsondump() method
 * @param pretty If true, format with indentation
 * @return JSON string
 */
template <typename T>
inline std::string json_dumps(const T &obj, bool pretty = true) {
  auto j = obj.jsondump();
  return pretty ? j.dump(2) : j.dump();
}

/**
 * @brief Deserialize object from JSON string.
 * @param json_str JSON string
 * @return Deserialized object
 */
template <typename T>
inline auto json_loads(const std::string &json_str)
    -> decltype(T::jsonload(std::declval<nlohmann::json>())) {
  auto j = nlohmann::json::parse(json_str);
  return T::jsonload(j);
}

/**
 * @brief Write object to JSON file (calls jsondump()).
 * @param obj Object with jsondump() method
 * @param filepath Path to output file
 * @param pretty If true, format with indentation
 */
template <typename T>
inline void json_dump(const T &obj, const std::string &filepath,
                      bool pretty = true) {
  json_dump(obj.jsondump(), filepath, pretty);
}

/**
 * @brief Read object from JSON file (calls jsonload()).
 * @param filepath Path to input file
 * @return Deserialized object
 */
template <typename T>
inline auto json_load(const std::string &filepath)
    -> decltype(T::jsonload(std::declval<nlohmann::json>())) {
  auto j = json_load_data(filepath);
  return T::jsonload(j);
}

/**
 * @brief Encode a collection of objects to JSON array.
 * @param collection Vector of objects with jsondump() method
 * @return JSON array
 */
template <typename T>
inline nlohmann::ordered_json encode_collection(const std::vector<T> &collection) {
  nlohmann::ordered_json arr = nlohmann::json::array();
  for (const auto &item : collection) {
    arr.push_back(item.jsondump());
  }
  return arr;
}

/**
 * @brief Encode a collection of shared pointers to JSON array.
 * @param collection Vector of shared pointers with jsondump() method
 * @return JSON array
 */
template <typename T>
inline nlohmann::ordered_json encode_collection(const std::vector<std::shared_ptr<T>> &collection) {
  nlohmann::ordered_json arr = nlohmann::json::array();
  for (const auto &item : collection) {
    if (item) {
      arr.push_back(item->jsondump());
    }
  }
  return arr;
}

/**
 * @brief Decode a JSON array to a collection of objects.
 * @param j JSON array
 * @return Vector of deserialized objects
 */
template <typename T>
inline std::vector<T> decode_collection(const nlohmann::json &j) {
  std::vector<T> result;
  if (j.is_array()) {
    for (const auto &item : j) {
      result.push_back(T::jsonload(item));
    }
  }
  return result;
}

/**
 * @brief Decode a JSON array to a collection of shared pointers.
 * @param j JSON array
 * @return Vector of shared pointers to deserialized objects
 */
template <typename T>
inline std::vector<std::shared_ptr<T>> decode_collection_ptr(const nlohmann::json &j) {
  std::vector<std::shared_ptr<T>> result;
  if (j.is_array()) {
    for (const auto &item : j) {
      result.push_back(std::make_shared<T>(T::jsonload(item)));
    }
  }
  return result;
}

} // namespace encoders
} // namespace session_cpp
