use core::num::NonZeroI32;
use esp_idf_svc::sys;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BLEError(NonZeroI32);

impl BLEError {
  pub fn fail() -> Result<(), Self> {
    Self::convert(0xFFFF)
  }

  pub const fn from_non_zero(error: NonZeroI32) -> Self {
    Self(error)
  }

  pub fn check_and_return<T>(error: u32, value: T) -> Result<T, Self> {
    match error {
      0 | sys::BLE_HS_EALREADY | sys::BLE_HS_EDONE => Ok(value),
      error => Err(Self(unsafe { NonZeroI32::new_unchecked(error as _) })),
    }
  }

  pub const fn convert(error: u32) -> Result<(), Self> {
    match error {
      0 | sys::BLE_HS_EALREADY | sys::BLE_HS_EDONE => Ok(()),
      error => Err(Self(unsafe { NonZeroI32::new_unchecked(error as _) })),
    }
  }

  pub fn code(&self) -> u32 {
    self.0.get() as _
  }
}

impl core::fmt::Display for BLEError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match return_code_to_string(self.0.get()) {
      Some(text) => write!(f, "{text}")?,
      None => write!(f, "0x{:X}", self.0)?,
    };

    Ok(())
  }
}

impl core::fmt::Debug for BLEError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match return_code_to_string(self.0.get()) {
      Some(text) => write!(f, "{text}")?,
      None => write!(f, "0x{:X}", self.0)?,
    };

    Ok(())
  }
}

#[cfg(feature = "std")]
impl std::error::Error for BLEError {}

pub fn return_code_to_string(rc: i32) -> Option<&'static str> {
  let val = rc as u32;

  const MAPPINGS: &[(u32, &str)] = &[
    (sys::BLE_HS_EALREADY, "Operation already in progress or completed."),
    (sys::BLE_HS_EINVAL, "One or more arguments are invalid."),
    (sys::BLE_HS_EMSGSIZE, "The provided buffer is too small."),
    (sys::BLE_HS_ENOENT, "No entry matching the specified criteria."),
    (sys::BLE_HS_ENOMEM, "Operation failed due to resource exhaustion."),
    (sys::BLE_HS_ENOTCONN, "No open connection with the specified handle."),
    (sys::BLE_HS_ENOTSUP, "Operation disabled at compile time."),
    (sys::BLE_HS_EAPP, "Application callback behaved unexpectedly."),
    (sys::BLE_HS_EBADDATA, "Command from peer is invalid."),
    (sys::BLE_HS_EOS, "Mynewt OS error."),
    (sys::BLE_HS_ECONTROLLER, "Event from controller is invalid."),
    (sys::BLE_HS_ETIMEOUT, "Operation timed out."),
    (sys::BLE_HS_EDONE, "Operation completed successfully."),
    (sys::BLE_HS_EBUSY, "Operation cannot be performed until procedure completes."),
    (sys::BLE_HS_EREJECT, "Peer rejected a connection parameter update request."),
    (sys::BLE_HS_EUNKNOWN, "Unexpected failure; catch all."),
    (sys::BLE_HS_EROLE, "Operation requires different role (e.g., central vs. peripheral)."),
    (sys::BLE_HS_ETIMEOUT_HCI, "HCI request timed out; controller unresponsive."),
    (
      sys::BLE_HS_ENOMEM_EVT,
      "Controller failed to send event due to memory exhaustion (combined host-controller only).",
    ),
    (
      sys::BLE_HS_ENOADDR,
      "Operation requires an identity address but none configured.",
    ),
    (
      sys::BLE_HS_ENOTSYNCED,
      "Attempt to use the host before it is synced with controller.",
    ),
    (sys::BLE_HS_EAUTHEN, "Insufficient authentication."),
    (sys::BLE_HS_EAUTHOR, "Insufficient authorization."),
    (sys::BLE_HS_EENCRYPT, "Insufficient encryption level."),
    (sys::BLE_HS_EENCRYPT_KEY_SZ, "Insufficient key size"),
    (sys::BLE_HS_ESTORE_CAP, "Storage at capacity."),
    (sys::BLE_HS_ESTORE_FAIL, "Storage IO error."),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INVALID_HANDLE,
      "The attribute handle given was not valid on this server.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_READ_NOT_PERMITTED,
      "The attribute cannot be read.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_WRITE_NOT_PERMITTED,
      "The attribute cannot be written.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INVALID_PDU,
      "The attribute PDU was invalid.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INSUFFICIENT_AUTHEN,
      "The attribute requires authentication before it can be read or written.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_REQ_NOT_SUPPORTED,
      "Attribute server does not support the request received from the client.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INVALID_OFFSET,
      "Offset specified was past the end of the attribute.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INSUFFICIENT_AUTHOR,
      "The attribute requires authorization before it can be read or written.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_PREPARE_QUEUE_FULL,
      "Too many prepare writes have been queued.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_ATTR_NOT_FOUND,
      "No attribute found within the given attribute handle range.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_ATTR_NOT_LONG,
      "The attribute cannot be read or written using the Read Blob Request.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INSUFFICIENT_KEY_SZ,
      "The Encryption Key Size used for encrypting this link is insufficient.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INVALID_ATTR_VALUE_LEN,
      "The attribute value length is invalid for the operation.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_UNLIKELY,
      "The attribute request has encountered an error that was unlikely, could not be completed as requested.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INSUFFICIENT_ENC,
      "The attribute requires encryption before it can be read or written.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_UNSUPPORTED_GROUP,
      "The attribute type is not a supported grouping attribute as defined by a higher layer specification.",
    ),
    (
      sys::BLE_HS_ERR_ATT_BASE + sys::BLE_ATT_ERR_INSUFFICIENT_RES,
      "Insufficient Resources to complete the request.",
    ),
    (
      sys::BLE_HS_ERR_HCI_BASE + sys::ble_error_codes_BLE_ERR_UNKNOWN_HCI_CMD,
      "Unknown HCI Command",
    ),
    (
      sys::BLE_HS_ERR_HCI_BASE + sys::ble_error_codes_BLE_ERR_UNK_CONN_ID,
      "Unknown Connection Identifier",
    ),
    (
      sys::BLE_HS_ERR_HCI_BASE + sys::ble_error_codes_BLE_ERR_AUTH_FAIL,
      "Authentication Failure",
    ),
    (
      sys::BLE_HS_ERR_HCI_BASE + sys::ble_error_codes_BLE_ERR_INV_HCI_CMD_PARMS,
      "Invalid HCI Command Parameters",
    ),
    (
      sys::BLE_HS_ERR_HCI_BASE + sys::ble_error_codes_BLE_ERR_REM_USER_CONN_TERM,
      "Remote User Terminated Connection",
    ),
    (
      sys::BLE_HS_ERR_HCI_BASE + sys::ble_error_codes_BLE_ERR_CONN_TERM_LOCAL,
      "Connection Terminated By Local Host",
    ),
    (
      sys::BLE_HS_ERR_HCI_BASE + sys::ble_error_codes_BLE_ERR_CONN_ESTABLISHMENT,
      "Connection Failed to be Established.",
    ),
    (
      sys::BLE_HS_ERR_L2C_BASE + sys::BLE_L2CAP_SIG_ERR_CMD_NOT_UNDERSTOOD,
      "Invalid or unsupported incoming L2CAP sig command.",
    ),
    (
      sys::BLE_HS_ERR_L2C_BASE + sys::BLE_L2CAP_SIG_ERR_MTU_EXCEEDED,
      "Incoming packet too large.",
    ),
    (
      sys::BLE_HS_ERR_L2C_BASE + sys::BLE_L2CAP_SIG_ERR_INVALID_CID,
      "No channel with specified ID.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_PASSKEY,
      "The user input of passkey failed, for example, the user cancelled the operation.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_OOB,
      "The OOB data is not available.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_AUTHREQ,
      "The pairing procedure cannot be performed as authentication requirements cannot be met due to IO capabilities of one or both devices.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_CONFIRM_MISMATCH,
      "The confirm value does not match the calculated compare value.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_PAIR_NOT_SUPP,
      "Pairing is not supported by the device.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_ENC_KEY_SZ,
      "The resultant encryption key size is insufficient for the security requirements of this device.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_CMD_NOT_SUPP,
      "The SMP command received is not supported on this device.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_UNSPECIFIED,
      "Pairing failed due to an unspecified reason.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_REPEATED,
      "Pairing or authentication procedure is disallowed because too little time has elapsed since last pairing request or security request.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_INVAL,
      "The Invalid Parameters error code indicates that the command length is invalid or that a parameter is outside of the specified range.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_DHKEY,
      "Indicates to the remote device that the DHKey Check value received doesn’t match the one calculated by the local device.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_NUMCMP,
      "Indicates that the confirm values in the numeric comparison protocol do not match.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_ALREADY,
      "Indicates that the pairing over the LE transport failed due to a Pairing Request sent over the BR/EDR transport in process.",
    ),
    (
      sys::BLE_HS_ERR_SM_US_BASE + sys::BLE_SM_ERR_CROSS_TRANS,
      "Indicates that the BR/EDR Link Key generated on the BR/EDR transport cannot be used to derive and distribute keys for the LE transport.",
    ),
  ];

  for (code, text) in MAPPINGS {
    if val == *code {
      return Some(text);
    }
  }

  None
}

#[cfg(not(feature = "debug"))]
macro_rules! ble {
  ($err:expr) => {{
    $crate::BLEError::convert($err as _)
  }};
}
#[cfg(feature = "debug")]
macro_rules! ble {
  ($err:expr) => {{
    let rc = $crate::BLEError::convert($err as _);
    if let Err(err) = rc {
      ::log::warn!(target: "esp32_nimble", "{}[{}]: {:?}", file!(), line!(), err);
    }
    rc
  }};
}

pub(crate) use ble;
