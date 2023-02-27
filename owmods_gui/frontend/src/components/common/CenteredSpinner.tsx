import Spinner, { SpinnerProps } from "@components/common/Spinner";

const CenteredSpinner = (props: SpinnerProps) => {
    return <Spinner {...props} className={`${props.className ?? ""} center`} />;
};

export default CenteredSpinner;
