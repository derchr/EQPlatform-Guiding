#include <memory>
#include <cstring>

#include "eq_platform.h"
#include "connectionplugins/connectionserial.h"



std::unique_ptr<EQController> eqController(new EQController());

// Return properties of device.
void ISGetProperties(const char *dev) {
    eqController->ISGetProperties(dev);
}

// Process new switch from client
void ISNewSwitch(const char *dev, const char *name, ISState *states, char *names[], int n)
{
    eqController->ISNewSwitch(dev, name, states, names, n);
}

// Process new text from client
void ISNewText(const char *dev, const char *name, char *texts[], char *names[], int n) {
    eqController->ISNewText(dev, name, texts, names, n);
}

// Process new number from client
void ISNewNumber(const char *dev, const char *name, double values[], char *names[], int n) {
    eqController->ISNewNumber(dev, name, values, names, n);
}

// Process new blob from client
void ISNewBLOB(const char *dev, const char *name, int sizes[], int blobsizes[], char *blobs[], char *formats[], char *names[], int n) {
    eqController->ISNewBLOB(dev, name, sizes, blobsizes, blobs, formats, names, n);
}

// Process snooped property from another driver
void ISSnoopDevice(XMLEle *root) {
    INDI_UNUSED(root);
}


// INDI is asking us for our default device name
const char* EQController::getDefaultName() {
    return "EQ Platform Driver";
}

// Client is asking us to establish connection to the device
bool EQController::Connect() {
    IDMessage(getDeviceName(), "EQ Platform Driver connected!");
    return DefaultDevice::Connect();
}

// Client is asking us to terminate connection to the device
bool EQController::Disconnect() {
    IDMessage(getDeviceName(), "EQ Platform Driver disconnected!");
    return DefaultDevice::Disconnect();
}

IPState EQController::GuideNorth(uint32_t ms) {
    // Guiding in DEC direction will not be supported.
    return IPS_IDLE;
}

IPState EQController::GuideSouth(uint32_t ms) {
    // Guiding in DEC direction will not be supported.
    return IPS_IDLE;
}

IPState EQController::GuideEast(uint32_t ms) {
    return IPS_BUSY;
}

IPState EQController::GuideWest(uint32_t ms) {
    return IPS_BUSY;
}

bool EQController::initProperties() {
    INDI::DefaultDevice::initProperties();

    initGuiderProperties(getDeviceName(), MOTION_TAB);

    setDriverInterface(AUX_INTERFACE | GUIDER_INTERFACE);

    // Add Debug, Simulation, and Configuration options to the driver
    addAuxControls();

    serialConnection = new Connection::Serial(this);
    serialConnection->registerHandshake([&]() { return Handshake(); });
    serialConnection->setDefaultBaudRate(Connection::Serial::B_57600);
    serialConnection->setDefaultPort("/dev/rfcomm0");
    registerConnection(serialConnection);

    return true;
}

bool EQController::updateProperties() {
    INDI::DefaultDevice::updateProperties();

    if (isConnected()) {
        defineNumber(&GuideNSNP);
        defineNumber(&GuideWENP);
    } else {
        deleteProperty(GuideNSNP.name);
        deleteProperty(GuideWENP.name);
    }

    return true;
}

bool EQController::Handshake() {
    if (isSimulation()) return true;
    PortFD = serialConnection->getPortFD();
    return true;
}


// prüfen: ist dise Funktion notwendig, damit phd die ms setzen kann?
bool EQController::ISNewNumber(const char *dev, const char *name, double values[], char *names[], int n) {
    if (!strcmp(name, GuideNSNP.name) || !strcmp(name, GuideWENP.name)) {
        processGuiderProperties(name, values, names, n);
        return true;
    }
    return INDI::DefaultDevice::ISNewNumber(dev, name, values, names, n);
}