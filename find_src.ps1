Get-ChildItem 'C:\pc\3_code\code_rust\session\session_cpp\src' -Filter '*.cpp' -Recurse | Select-Object Name, FullName | Sort-Object Name
