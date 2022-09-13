import { useCallback, useContext, useState } from "react";
import {
  Button,
  Box,
  Container,
  Paper,
  TextField,
  Typography,
  CircularProgress,
} from "@mui/material";
import { tauri } from "../tauri";
import { ApplicationContext, ApplicationContextType } from "../context";
import { ApplicationErrorCode } from "../constants";

const Authentication = () => {
  const { setDevice, setError } =
    useContext<ApplicationContextType>(ApplicationContext);

  const [loading, setLoading] = useState(false);

  const [accessKeyId, setAccessKeyId] = useState("");
  const [secretAccessKey, setSecretAccessKey] = useState("");

  const authenticate = useCallback(() => {
    setLoading(true);

    tauri
      .authenticate(accessKeyId, secretAccessKey)
      .then((device) => {
        setDevice(device);
        setLoading(false);
      })
      .catch(() => {
        setError({
          code: ApplicationErrorCode.InitializationError,
          message: "Authentication failed",
        });
        setLoading(false);
      });
  }, [accessKeyId, secretAccessKey]);

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
            <Button variant="outlined" onClick={authenticate}>
              Authenticate
            </Button>
          </Paper>
        </>
      )}
    </Container>
  );
};

export default Authentication;
