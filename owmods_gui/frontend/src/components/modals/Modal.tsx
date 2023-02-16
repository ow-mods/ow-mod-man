import { useTranslations } from "@hooks";
import { MutableRefObject, ReactNode, useEffect, useState } from "react";
import { IconContext } from "react-icons";

export interface ModalProps {
    heading?: string;
    confirmText?: string;
    showCancel?: boolean;
    cancelText?: string;
    open?: MutableRefObject<() => void>;
    close?: MutableRefObject<() => void>;
    children: ReactNode;
    onCancel?: () => boolean | void;
    onConfirm?: () => boolean | void;
}

export interface ModalWrapperProps {
    open?: MutableRefObject<() => void>;
}

interface OpenState {
    open: boolean;
    closing: boolean;
}

const Modal = (props: ModalProps) => {
    const [state, setState] = useState<OpenState>({ open: false, closing: false });
    const [awaitingClose, setAwaitingClose] = useState(false);

    const open = () => setState({ open: true, closing: false });
    const close = () => {
        setAwaitingClose(false);
        setState({ open: true, closing: true });
    };

    if (props.open) {
        props.open.current = open;
    }

    if (props.close) {
        props.close.current = close;
    }

    const [cancel, ok] = useTranslations(["CANCEL", "OK"]);

    useEffect(() => {
        if (state.open) {
            document.documentElement.classList.add("modal-is-opening", "modal-is-open");
            setTimeout(() => {
                document.documentElement.classList.remove("modal-is-opening");
            }, 1000);
        } else {
            document.documentElement.classList.remove("modal-is-closing");
        }
        if (state.closing) {
            document.documentElement.classList.remove("modal-is-open");
            document.documentElement.classList.add("modal-is-closing");
            setTimeout(() => {
                setState({ open: false, closing: false });
            }, 1000);
        }
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
};

export default Modal;
