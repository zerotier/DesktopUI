import React, { createElement } from 'react';
import ReactDOM from 'react-dom';

import { EuiPanel, EuiPageTemplate, EuiText, EuiResizableContainer } from '@elastic/eui';

ReactDOM.render((
  <EuiResizableContainer style={{ height: "100vh", maxHeight: "100vh", minHeight: "100vh"}}>{(EuiResizablePanel, EuiResizableButton) => (<>
    <EuiResizablePanel initialSize={50} minSize="30%" grow={true}>
      <EuiText>
        panel 1
      </EuiText>
    </EuiResizablePanel>
    <EuiResizableButton />
    <EuiResizablePanel initialSize={50} minSize="30%" grow={true}>
      <EuiText>
        panel 2
      </EuiText>
    </EuiResizablePanel>
  </>)}</EuiResizableContainer>
), document.getElementById("_app_root"));
