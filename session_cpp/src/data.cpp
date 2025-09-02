// #ifndef DATA_HPP
// #define DATA_HPP

// #include <iostream>
// #include <string>
// #define UUID_SYSTEM_GENERATOR
// #include "uuid_v4.h"
// #include "json.hpp"

// using json = nlohmann::json;

// namespace ns
// {
//     struct person
//     {
//         std::string name;
//         std::string address;
//         int age;
//     };

//     void to_json(json &j, const person &p)
//     {
//         j = json{{"name", p.name}, {"address", p.address}, {"age", p.age}};
//         UUIDv4::UUIDGenerator<std::mt19937_64> uuidGenerator;
//         UUIDv4::UUID uuid = uuidGenerator.getUUID();
//     }

//     void from_json(const json &j, person &p)
//     {
//         j.at("name").get_to(p.name);
//         j.at("address").get_to(p.address);
//         j.at("age").get_to(p.age);
//     }
// } // namespace ns

// /**
//  * @brief Base data structure for all objects for serializetion to json.
//  *
//  */
// class Data
// {
// public:
//     /**
//      * @brief Construct a new Data object
//      *
//      * @param name The name of the object.
//      */
//     Data(const std::string &name = "");

//     /**
//      * @brief Desctructor for the Data object.
//      */
//     virtual ~Data();

//     /**
//      * @brief Getter for GUID.
//      *
//      * @return The GUID of the object.
//      */
//     std::string guid() const;

//     /**
//      * @brief Getter for the name of the object.
//      *
//      * @return The name of the object.
//      */
//     std::string name() const;

//     /**
//      * @brief Setter for the name of the object.
//      *
//      * @param name The name of the object.
//      */
//     void name(const std::string &name);

//     /**
//      * @brief Abstract method to get the data type.
//      *
//      * @return The data type of the object.
//      */
//     virtual std::string dtype() const = 0;

// protected:
//     std::string _guid; /**< The GUID of the object. */
//     std::string _name; /**< The name of the object. */
// };

// #endif // DATA_HPP