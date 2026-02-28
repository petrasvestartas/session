$raw = Get-Content 'C:\pc\3_code\code_rust\session\session_tests\session_cpp\mesh_test.json' -Raw
$raw | Select-Object -First 1
$raw.Substring(0, 800)
