// based on https://gist.github.com/chibicode/fe195d792270910226c928b69a468206

import React, { memo } from 'react';
import twemoji from 'twemoji';

interface TwemojiProps {
  emoji: string;
}

const Twemoji: React.FC<TwemojiProps> = ({ emoji }) => (
  <span
    dangerouslySetInnerHTML={{
      __html: twemoji.parse(emoji, {
        folder: 'svg',
        ext: '.svg'
      })
    }}
  />
);

export default memo(Twemoji);