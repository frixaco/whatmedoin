#include <windows.h>
#include <psapi.h>
#include <stdio.h>

__declspec(dllexport) const char *get_foreground_app_win()
{
    static char result[512];
    HWND hwnd = GetForegroundWindow();
    if (hwnd)
    {
        char window_title[256];
        GetWindowTextA(hwnd, window_title, sizeof(window_title));

        DWORD process_id;
        GetWindowThreadProcessId(hwnd, &process_id);

        HANDLE process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, process_id);
        if (process)
        {
            char process_name[MAX_PATH];
            if (GetModuleFileNameExA(process, NULL, process_name, sizeof(process_name)))
            {
                snprintf(result, sizeof(result), "Window: %s, Process: %s", window_title, process_name);
            }
            else
            {
                snprintf(result, sizeof(result), "Failed to get process name");
            }
            CloseHandle(process);
        }
        else
        {
            snprintf(result, sizeof(result), "Failed to open process");
        }
    }
    else
    {
        snprintf(result, sizeof(result), "No foreground window found");
    }

    return result;
}