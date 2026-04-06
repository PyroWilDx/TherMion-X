#include "ADLXHelper/Windows/Cpp/ADLXHelper.h"
#include "Include/IPerformanceMonitoring.h"

#include <Windows.h>

using namespace adlx;

static ADLXHelper adlxHelper = ADLXHelper();

extern "C" {
int ADLX_Init() {
    return ADLX_SUCCEEDED(adlxHelper.Initialize()) ? 0 : 1;
}

int ADLX_InitPerformanceService(void** res) {
    IADLXSystem* sysSvc = adlxHelper.GetSystemServices();
    if (!sysSvc)
        return -1;

    IADLXPerformanceMonitoringServicesPtr perfMonitor;
    if (ADLX_FAILED(sysSvc->GetPerformanceMonitoringServices(&perfMonitor)))
        return -1;

    *res = static_cast<void*>(perfMonitor);

    return 0;
}

int ADLX_GetGPUNames(char*** resNames, int* resCount) {
    IADLXSystem* sysSvc = adlxHelper.GetSystemServices();
    if (!sysSvc)
        return -1;

    IADLXGPUListPtr sysGPUs;
    if (ADLX_FAILED(sysSvc->GetGPUs(&sysGPUs)) || sysGPUs == nullptr)
        return -1;

    char** names = static_cast<char**>(malloc(sizeof(char*) * sysGPUs->Size()));
    const int count = static_cast<int>(sysGPUs->Size());

    for (int i = 0; i < count; i++) {
        IADLXGPUPtr currGPU;
        if (ADLX_FAILED(sysGPUs->At(i, &currGPU))) {
            return -1;
        }

        const char* name = nullptr;
        if (ADLX_FAILED(currGPU->Name(&name)) || name == nullptr) {
            return -1;
        }

        names[i] = static_cast<char*>(malloc(sizeof(char) * (strlen(name) + 1)));
        strcpy_s(names[i], strlen(name) + 1, name);
    }

    *resNames = names;
    *resCount = count;

    return 0;
}

int ADLX_FreeGPUNames(char** names, const int count) {
    for (int i = 0; i < count; i++) {
        free(names[i]);
    }

    free(names);

    return 0;
}

int ADLX_GetGPU(void** res, const char* name) {
    IADLXSystem* sysSvc = adlxHelper.GetSystemServices();
    if (!sysSvc)
        return -1;

    IADLXGPUListPtr sysGPUs;
    if (ADLX_FAILED(sysSvc->GetGPUs(&sysGPUs)) || sysGPUs == nullptr)
        return -1;

    for (int i = 0; i < sysGPUs->Size(); i++) {
        IADLXGPUPtr currGPU;
        if (ADLX_FAILED(sysGPUs->At(i, &currGPU))) {
            return -1;
        }

        const char* currName = nullptr;
        if (ADLX_FAILED(currGPU->Name(&currName)) || currName == nullptr) {
            return -1;
        }

        if (strcmp(currName, name) == 0) {
            *res = static_cast<void*>(currGPU);

            return 0;
        }
    }

    return -1;
}
}
