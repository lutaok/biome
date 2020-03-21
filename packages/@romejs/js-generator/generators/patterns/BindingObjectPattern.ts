/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  BindingObjectPattern,
  bindingObjectPattern,
  AnyNode,
} from '@romejs/js-ast';
import ObjectExpression from '../objects/ObjectExpression';
import {printPatternMeta} from '../utils';

export default function BindingObjectPattern(generator: Generator, node: AnyNode) {
  node = bindingObjectPattern.assert(node);

  ObjectExpression(generator, node);
  printPatternMeta(generator, node, node.meta);
}
