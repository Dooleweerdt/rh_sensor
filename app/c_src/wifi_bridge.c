#include <app/wifi_settings.h>

#ifdef WIFI_SSID
void * wifi_bridge_get_ssid(void) {
    #ifdef WIFI_CREDENTIALS
    return WIFI_SSID;
    #else
    return "WifiSSID";
    #endif
}
#endif

#ifdef WIFI_PSK
void * wifi_bridge_get_psk(void) {
    #ifdef WIFI_CREDENTIALS
    return WIFI_PSK;
    #else
    return "WifiPSK"
    #endif
}
#endif
