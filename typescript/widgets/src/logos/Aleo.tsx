import React, { SVGProps, memo } from 'react';

function _AleoLogo(props: SVGProps<SVGSVGElement>) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 100 100"
      fill="none"
      {...props}
    >
      <circle cx="50" cy="50" r="50" fill="#0B0B0F" />
      <path
        fill="#FFFFFF"
        d="M50 16L22 86h12l6-16h20l6 16h12L50 16Zm-6.5 44L50 41l6.5 19H43.5Z"
      />
    </svg>
  );
}

export const AleoLogo = memo(_AleoLogo);

