#include "ADLXHelper/Windows/Cpp/ADLXHelper.h"
#include "Include/IPerformanceMonitoring.h"

#include <Windows.h>

using namespace adlx;

static ADLXHelper adlxHelper = ADLXHelper();
static bool adlxInitialized = false;

static bool Init() {
    if (!adlxInitialized)
        adlxInitialized = ADLX_SUCCEEDED(adlxHelper.Initialize());

    return adlxInitialized;
}

extern "C"
int GetGPUUsage(const int index) {
    if (!Init())
        return -1;

    IADLXSystem* sysSvc = adlxHelper.GetSystemServices();
    if (!sysSvc)
        return -1;

    IADLXPerformanceMonitoringServicesPtr perfMonitor;
    if (ADLX_FAILED(sysSvc->GetPerformanceMonitoringServices(&perfMonitor)))
        return -1;

    IADLXGPUListPtr sysGPUs;
    if (ADLX_FAILED(sysSvc->GetGPUs(&sysGPUs)))
        return -1;

    IADLXGPUPtr sysGPU;
    if (ADLX_FAILED(sysGPUs->At(index, &sysGPU)))
        return -1;

    IADLXAllMetricsPtr allMetrics;
    if (ADLX_FAILED(perfMonitor->GetCurrentAllMetrics(&allMetrics)))
        return -1;

    IADLXGPUMetricsPtr sysGPUMetrics;
    if (ADLX_FAILED(allMetrics->GetGPUMetrics(sysGPU, &sysGPUMetrics)))
        return -1;

    adlx_double sysGPUUsage = 0;
    if (ADLX_FAILED(sysGPUMetrics->GPUUsage(&sysGPUUsage)))
        return -1;

    return static_cast<int>(sysGPUUsage);
}
