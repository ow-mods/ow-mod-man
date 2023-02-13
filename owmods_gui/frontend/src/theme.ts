import { Theme } from "@types";

import blue from "@styles/themes/blue.scss?inline";
import blurple from "@styles/themes/blurple.scss?inline";
import ghostGreen from "@styles/themes/ghostGreen.scss?inline";
import green from "@styles/themes/green.scss?inline";
import orange from "@styles/themes/orange.scss?inline";
import pink from "@styles/themes/pink.scss?inline";
import white from "@styles/themes/white.scss?inline";
import yellow from "@styles/themes/yellow.scss?inline";

const ThemeMap: Record<Theme, string> = {
    Blue: blue,
    Blurple: blurple,
    GhostlyGreen: ghostGreen,
    Green: green,
    Orange: orange,
    Pink: pink,
    White: white,
    Yellow: yellow
};

export default ThemeMap;
