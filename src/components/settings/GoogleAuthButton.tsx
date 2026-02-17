interface GoogleAuthButtonProps {
  isConnected: boolean;
  onClick: () => void;
}

export function GoogleAuthButton({
  isConnected,
  onClick
}: GoogleAuthButtonProps) {
  return (
    <button onClick={onClick}>
      {isConnected ? 'Google Connected' : 'Connect Google'}
    </button>
  );
}
