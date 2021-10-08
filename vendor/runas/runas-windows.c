#include <Windows.h>


DWORD rust_win_runas(LPCTSTR *cmd, LPCTSTR *params, int show)
{
    DWORD code;
    SHELLEXECUTEINFO sei = { sizeof(sei) };

    CoInitializeEx(NULL, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);

    sei.fMask = SEE_MASK_NOASYNC | SEE_MASK_NOCLOSEPROCESS;
    sei.lpVerb = L"runas";
    sei.lpFile = cmd;
    sei.lpParameters = params;
    sei.nShow = show ? SW_NORMAL : SW_HIDE;

    if (ShellExecuteExW(&sei) == FALSE || sei.hProcess == NULL) {
        return -1;
    }

    WaitForSingleObject(sei.hProcess, INFINITE);

    if (GetExitCodeProcess(sei.hProcess, &code) == 0) {
        return -1;
    }

    return code;
}
