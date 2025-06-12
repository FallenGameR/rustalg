# Rand Data

```ps1
cd $PsScriptRoot
$source = "https://algs4.cs.princeton.edu/14analysis"
$files = "1Kints.txt", "2Kints.txt", "4Kints.txt", "8Kints.txt", "16Kints.txt", "32Kints.txt", "1Mints.txt"
$files | foreach{ http "$source/$_" > $_ }
```
