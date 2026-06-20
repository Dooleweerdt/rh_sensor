#include <app/wifi_settings.h>

void * wifi_bridge_get_ssid(void) {
    #ifdef WIFI_CREDENTIALS
    return WIFI_SSID;
    #else
    return "WifiSSID";
    #endif
}

void * wifi_bridge_get_psk(void) {
    #ifdef WIFI_CREDENTIALS
    return WIFI_PSK;
    #else
    return "WifiPSK"
    #endif
}
