import { useTranslations } from "@hooks";
import { ReactNode, forwardRef, useEffect, useImperativeHandle, useState } from "react";
import { IconContext } from "react-icons";

export interface ModalProps {
    heading?: string;
    confirmText?: string;
    showCancel?: boolean;
    cancelText?: string;
    children: ReactNode;
    onCancel?: () => boolean | void;
    onConfirm?: () => boolean | void;
}

export interface ModalHandle {
    open: () => void;
    close: () => void;
}

interface OpenState {
    open: boolean;
    closing: boolean;
}

const Modal = forwardRef(function Modal(props: ModalProps, ref) {
    const [state, setState] = useState<OpenState>({ open: false, closing: false });
    const [awaitingClose, setAwaitingClose] = useState(false);

    useImperativeHandle(
        ref,
        () => ({
            open: () => setState({ open: true, closing: false }),
            close: () => {
                setAwaitingClose(false);
                setState({ open: true, closing: true });
            }
        }),
        []
    );

    const [cancel, ok] = useTranslations(["CANCEL", "OK"]);

    useEffect(() => {
        let timeout: number;
        if (state.open) {
            document.documentElement.classList.add("modal-is-opening", "modal-is-open");
            timeout = setTimeout(() => {
                document.documentElement.classList.remove("modal-is-opening");
            }, 1000);
        } else {
            document.documentElement.classList.remove("modal-is-closing");
        }
        if (state.closing) {
            document.documentElement.classList.remove("modal-is-open");
            document.documentElement.classList.add("modal-is-closing");
            timeout = setTimeout(() => {
                setState({ open: false, closing: false });
            }, 1000);
        }
        return () => {
            clearTimeout(timeout);
        };
    }, [state]);

    return (
        <dialog className={state.open ? "" : "d-none"} dir="ltr" open={state.open}>
            <IconContext.Provider value={{ className: "modal-icon" }}>
                <article>
                    <header>
                        <p>{props.heading ?? "Modal"}</p>
                    </header>
                    <div className="modal-body">{props.children}</div>
                    <footer>
                        {props.showCancel && (
                            <a
                                href="#cancel"
                                role="button"
                                className="secondary"
                                onClick={() => {
                                    if (props.onCancel?.() ?? true) {
                                        close();
                                    }
                                }}
                            >
                                {props.cancelText ?? cancel}
                            </a>
                        )}
                        <a
                            href="#confirm"
                            role="button"
                            aria-busy={awaitingClose}
                            onClick={() => {
                                setAwaitingClose(true);
                                if (props.onConfirm?.() ?? true) {
                                    close();
                                }
                            }}
                        >
                            {!awaitingClose && (props.confirmText ?? ok)}
                        </a>
                    </footer>
                </article>
            </IconContext.Provider>
        </dialog>
    );
});

export default Modal;
