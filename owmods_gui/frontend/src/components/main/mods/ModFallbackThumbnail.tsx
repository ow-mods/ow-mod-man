import { Box } from "@mui/material";
import { memo } from "react";

export type ModFallbackThumbnailProps = {
    modName: string;
    fallbackUrl: string;
    className?: string;
};

const stringToNumber = (str: string, seed = 3) => {
    let h1 = 0xdeadbeef ^ seed,
        h2 = 0x41c6ce57 ^ seed;
    for (let i = 0, ch; i < str.length; i++) {
        ch = str.charCodeAt(i);
        h1 = Math.imul(h1 ^ ch, 2654435761);
        h2 = Math.imul(h2 ^ ch, 1597334677);
    }
    h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909);
    h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909);
    return 4294967296 * (2097151 & h2) + (h1 >>> 0);
};

const getHueFromText = (text: string): string => `hue-rotate(${stringToNumber(text) % 360}deg)`;

const ModFallbackThumbnail = memo(function ModFallbackThumbnail(props: ModFallbackThumbnailProps) {
    return (
        <div style={{ margin: 0, padding: 0, position: "relative" }}>
            <Box
                height="100%"
                width="100%"
                zIndex={1}
                display="flex"
                alignItems="center"
                justifyContent="center"
                position="absolute"
                right={0}
                left={0}
                padding={1}
                fontWeight="bold"
                textAlign="center"
            >
                <p>{props.modName}</p>
            </Box>
            <img
                width="450"
                height="150"
                style={{ width: "100%", filter: getHueFromText(props.modName) }}
                src={props.fallbackUrl}
                className={`mod-thumb fallback ${props.className ?? ""}`}
            />
        </div>
    );
});

export default ModFallbackThumbnail;
