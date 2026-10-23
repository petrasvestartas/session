$ErrorActionPreference = 'Stop'
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class WarpLuid {
    [DllImport("d3d11.dll")] static extern int D3D11CreateDevice(IntPtr adapter, int driver, IntPtr software, uint flags, IntPtr levels, uint count, uint sdk, out IntPtr device, out int level, out IntPtr context);
    [UnmanagedFunctionPointer(CallingConvention.StdCall)] delegate int GetAdapter(IntPtr self, out IntPtr adapter);
    [UnmanagedFunctionPointer(CallingConvention.StdCall)] delegate int GetDesc(IntPtr self, IntPtr description);
    public static string Read() {
        IntPtr device = IntPtr.Zero, context = IntPtr.Zero, dxgi = IntPtr.Zero, adapter = IntPtr.Zero, desc = IntPtr.Zero;
        try {
            int level;
            Marshal.ThrowExceptionForHR(D3D11CreateDevice(IntPtr.Zero, 5, IntPtr.Zero, 0, IntPtr.Zero, 0, 7, out device, out level, out context));
            Guid iid = new Guid("54ec77fa-1377-44e6-8c32-88fd5f44c84c");
            Marshal.ThrowExceptionForHR(Marshal.QueryInterface(device, ref iid, out dxgi));
            var getAdapter = (GetAdapter)Marshal.GetDelegateForFunctionPointer(Marshal.ReadIntPtr(Marshal.ReadIntPtr(dxgi), 7 * IntPtr.Size), typeof(GetAdapter));
            Marshal.ThrowExceptionForHR(getAdapter(dxgi, out adapter));
            var getDesc = (GetDesc)Marshal.GetDelegateForFunctionPointer(Marshal.ReadIntPtr(Marshal.ReadIntPtr(adapter), 8 * IntPtr.Size), typeof(GetDesc));
            desc = Marshal.AllocHGlobal(512);
            Marshal.ThrowExceptionForHR(getDesc(adapter, desc));
            string name = Marshal.PtrToStringUni(desc);
            uint vendor = unchecked((uint)Marshal.ReadInt32(desc, 256));
            uint id = unchecked((uint)Marshal.ReadInt32(desc, 260));
            if (vendor != 0x1414 || id != 0x8c || !name.Contains("Microsoft Basic Render Driver")) throw new Exception("Not WARP: " + name);
            int offset = 272 + 3 * IntPtr.Size;
            uint low = unchecked((uint)Marshal.ReadInt32(desc, offset));
            int high = Marshal.ReadInt32(desc, offset + 4);
            return "{\"name\":\"" + name + "\",\"vendor\":" + vendor + ",\"device\":" + id + ",\"luid\":\"" + high + "," + low + "\"}";
        } finally {
            if (desc != IntPtr.Zero) Marshal.FreeHGlobal(desc);
            foreach (IntPtr item in new[] {adapter, dxgi, context, device}) if (item != IntPtr.Zero) Marshal.Release(item);
        }
    }
}
'@
[WarpLuid]::Read()
