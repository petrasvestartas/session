// #include "Data.hpp"

// /**
//  * @brief Construct a new Data object
//  *
//  * @param name The name of the object.
//  */

// Data::Data(const std::string &name) : _name(name)
// {
//     uuid_t uuid;
//     uuid_generate_random(uuid);
//     char uuid_str[37];
//     uuid_unparse_lower(uuid, uuid_str);
//     _guid = uuid_str
// }

// /**
//  * @brief Destructor
//  */
// Data::~Data() {}

// /**
//  * @brief Getter for GUID.
//  *
//  * @return The GUID of the object.
//  */
// std::string Data::guid() const
// {
//     return _guid;
// }

// /**
//  * @brief Getter for the name of the object.
//  *
//  * @return The name of the object.
//  */
// std::string Data::name() const
// {
//     return _name;
// }

// /**
//  * @brief Setter for the name of the object.
//  *
//  * @param name The name of the object.
//  */
// void Data::name(const std::string &name)
// {
//     _name = name;
