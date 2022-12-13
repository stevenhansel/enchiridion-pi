import { Paper, Modal, Typography, Box } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { useCallback, useContext, useEffect, useState } from "react";
import { ApplicationErrorCode } from "../constants";
import { ApplicationContext, ApplicationContextType } from "../context";
import { isTauriErrorObject, tauri, TauriErrorObject } from "../tauri";

type Props = {
  open: boolean;
  handleClose: () => void;
};

const ApplicationSettings = ({ open, handleClose }: Props) => {
  const { device, setDevice, setError } =
    useContext<ApplicationContextType>(ApplicationContext);

  const [confirmUnlink, setConfirmUnlink] = useState(false);
  const [loading, setLoading] = useState(false);

  const unlinkDevice = useCallback(async () => {
    try {
      if (!confirmUnlink) {
        setConfirmUnlink(true);
        return;
      }

      setLoading(true);

      const response = await tauri.unlink();
      if (isTauriErrorObject<void>(response)) {
        let { errorCode, messages } = response as TauriErrorObject;
        setError({ code: errorCode, message: messages[0] });
        return;
      }

      setLoading(false);
      setDevice(null);
    } catch (err) {
      setError({
        code: ApplicationErrorCode.ApplicationError,
        message: "Something when wrong when unlinking the device",
      });
    }
  }, [confirmUnlink]);

  useEffect(() => {
    if (!open && confirmUnlink) {
      setConfirmUnlink(false);
    }
  }, [open]);

  return (
    <Modal open={open} onClose={handleClose}>
      <Paper
        sx={{
          position: "absolute",
          top: "50%",
          left: "50%",
          transform: "translate(-50%, -50%)",
          width: 1000,
          height: 600,
          outline: 0,
        }}
      >
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            paddingX: 6,
            paddingY: 4,
          }}
        >
          {device ? (
            <Box sx={{ marginBottom: 2 }}>
              <Typography variant="h6" sx={{ fontWeight: 600, marginBottom: 1 }}>Device</Typography>

              <Box sx={{ marginBottom: 1 }}>
                <Typography sx={{ fontWeight: 600 }}>ID:</Typography>
                <Typography>{device.id}</Typography>
              </Box>

              <Box sx={{ marginBottom: 1 }}>
                <Typography sx={{ fontWeight: 600 }}>Name:</Typography>
                <Typography>{device.name}</Typography>
              </Box>

              <Box sx={{ marginBottom: 1 }}>
                <Typography sx={{ fontWeight: 600 }}>Description:</Typography>
                <Typography>{device.description}</Typography>
              </Box>

              <Box sx={{ marginBottom: 1 }}>
                <Typography sx={{ fontWeight: 600 }}>Location:</Typography>
                <Typography>{device.location}</Typography>
              </Box>
            </Box>
          ) : null}
          <Box
            sx={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            }}
          >
            <Box>
              <Typography variant="h6" sx={{ fontWeight: 600 }}>
                Unlink
              </Typography>
              <Typography sx={{ marginBottom: 2 }}>
                Unlinking is experimental, please use with caution
              </Typography>
            </Box>

            <LoadingButton
              variant="contained"
              loading={loading}
              onClick={unlinkDevice}
            >
              {confirmUnlink
                ? "Click again to confirm device unlink"
                : "Unlink Device"}
            </LoadingButton>
          </Box>
        </Box>
      </Paper>
    </Modal>
  );
};

export default ApplicationSettings;
