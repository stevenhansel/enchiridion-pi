import { useCallback, useEffect, useContext, useState } from "react";
import {
  Button,
  Box,
  Container,
  Paper,
  TextField,
  Typography,
  CircularProgress,
} from "@mui/material";
import {
  DeviceInformation,
  isTauriErrorObject,
  tauri,
  TauriErrorObject,
} from "../tauri";
import { getMatches } from '@tauri-apps/api/cli';
import { ApplicationContext, ApplicationContextType } from "../context";
import { ApplicationErrorCode } from "../constants";

const Authentication = () => {
  const { setDevice, setError } =
    useContext<ApplicationContextType>(ApplicationContext);

  const [loading, setLoading] = useState(false);

  const [accessKeyId, setAccessKeyId] = useState("");
  const [secretAccessKey, setSecretAccessKey] = useState("");
  const [cameraEnabled, setCameraEnabled] = useState(false);

  const link = useCallback(async () => {
    try {
      setLoading(true);

      const response = await tauri.link(accessKeyId, secretAccessKey, cameraEnabled);
      if (isTauriErrorObject<DeviceInformation>(response)) {
        let { errorCode, messages } = response as TauriErrorObject;
        setError({ code: errorCode, message: messages[0] });

        setLoading(false);
        return;
      }

      setDevice(response as DeviceInformation);
    } catch {
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Authentication failed",
      });
      setLoading(false);
    }
  }, [accessKeyId, secretAccessKey]);

  useEffect(() => {
console.log('asd');
    getMatches().then((matches) => {
	console.log('matches: ', matches);
    })
  }, []);

  return (
    <Container
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        width: "100%",
        height: "80vh",
      }}
    >
      {loading ? (
        <CircularProgress />
      ) : (
        <>
          <Box
            sx={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              mb: 7,
            }}
          >
            <Typography sx={{ typography: "h3" }}>Enchiridion</Typography>
            <Typography sx={{ typography: "h6" }}>
              Computer Engineering BINUS
            </Typography>
          </Box>

          <Paper
            variant="outlined"
            sx={{
              display: "flex",
              flexDirection: "column",
              justifyContent: "space-between",
              width: 512,
              height: 200,
            }}
          >
            <Box>
              <TextField
                fullWidth
                sx={{ marginBottom: 2 }}
                label="Access Key ID"
                autoComplete="off"
                variant="outlined"
                value={accessKeyId}
                onChange={(e) => setAccessKeyId(e.target.value)}
              />
              <TextField
                fullWidth
                label="Secret Access Key"
                autoComplete="off"
                variant="outlined"
                value={secretAccessKey}
                onChange={(e) => setSecretAccessKey(e.target.value)}
              />
            </Box>
            <Button variant="outlined" onClick={link}>
              Authenticate
            </Button>
          </Paper>
        </>
      )}
    </Container>
  );
};

export default Authentication;
