import { useState } from "react";
import Tooltip, { TooltipProps } from "@mui/material/Tooltip";

const ODTooltip = ({ children, ...rest }: TooltipProps) => {
    const [renderTooltip, setRenderTooltip] = useState(false);

    return (
        <div
            onMouseLeave={() => renderTooltip && setRenderTooltip(false)}
            onMouseEnter={() => !renderTooltip && setRenderTooltip(true)}
        >
            {!renderTooltip && children}
            {renderTooltip && (
                <Tooltip disableInteractive enterDelay={350} {...rest}>
                    {children}
                </Tooltip>
            )}
        </div>
    );
};

export default ODTooltip;
