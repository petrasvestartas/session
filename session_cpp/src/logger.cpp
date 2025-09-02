#include "logger.hpp"

#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

namespace geo {
class Logger {
   public:
    Logger() {
        logFile.open("app.log", std::ios::app);  // Append mode
    }

    ~Logger() {
        if (logFile.is_open()) {
            logFile.close();
        }
    }

    static std::ofstream& getLogFile() {
        static Logger instance;  // Static instance ensures initialization and cleanup
        return instance.logFile;
    }

   private:
    std::ofstream logFile;
};

void log(const std::string& message, LogLevel color) {
    auto& logFile = Logger::getLogFile();
    std::string formattedMessage;
    if (color != LogLevel::DEFAULT) {
        formattedMessage =
            "\033[" + std::to_string(static_cast<int>(color)) + "m" + "   " + message + "\033[0m";
    } else {
        formattedMessage = "   " + message;
    }

    if (logFile.is_open()) {
        logFile << formattedMessage << std::endl;
    } else {
        std::cerr << "Logger not initialized or file cannot be opened." << std::endl;
    }

    // Also print the message to the console with color
    std::cout << formattedMessage << std::endl;
}

std::string readFullLog(const std::string& logFilePath) {
    std::ifstream logFile(logFilePath);
    std::stringstream logStream;

    if (logFile.is_open()) {
        logStream << logFile.rdbuf();  // Read the whole file into the stringstream
        logFile.close();
    } else {
        return "Logger file could not be opened.";
    }

    return logStream.str();  // Convert the stringstream to string and return
}

}  // namespace geo