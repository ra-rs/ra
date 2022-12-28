#[doc = "Register `BCTL` reader"]
pub struct R(crate::R<BCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCTL` writer"]
pub struct W(crate::W<BCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCBA` reader - Include I3C Broadcast Address"]
pub type INCBA_R = crate::BitReader<INCBA_A>;
#[doc = "Include I3C Broadcast Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INCBA_A {
    #[doc = "0: Do not include I3C broadcast address for private transfers"]
    _0 = 0,
    #[doc = "1: Include I3C broadcast address for private transfers"]
    _1 = 1,
}
impl From<INCBA_A> for bool {
    #[inline(always)]
    fn from(variant: INCBA_A) -> Self {
        variant as u8 != 0
    }
}
impl INCBA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INCBA_A {
        match self.bits {
            false => INCBA_A::_0,
            true => INCBA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INCBA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INCBA_A::_1
    }
}
#[doc = "Field `INCBA` writer - Include I3C Broadcast Address"]
pub type INCBA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCTL_SPEC, INCBA_A, O>;
impl<'a, const O: u8> INCBA_W<'a, O> {
    #[doc = "Do not include I3C broadcast address for private transfers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INCBA_A::_0)
    }
    #[doc = "Include I3C broadcast address for private transfers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INCBA_A::_1)
    }
}
#[doc = "Field `HJACKCTL` reader - Hot-Join Acknowledge Control"]
pub type HJACKCTL_R = crate::BitReader<HJACKCTL_A>;
#[doc = "Hot-Join Acknowledge Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HJACKCTL_A {
    #[doc = "0: ACK the Hot-Join request"]
    _0 = 0,
    #[doc = "1: NACK and send broadcast CCC to disable Hot-Join"]
    _1 = 1,
}
impl From<HJACKCTL_A> for bool {
    #[inline(always)]
    fn from(variant: HJACKCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl HJACKCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HJACKCTL_A {
        match self.bits {
            false => HJACKCTL_A::_0,
            true => HJACKCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HJACKCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HJACKCTL_A::_1
    }
}
#[doc = "Field `HJACKCTL` writer - Hot-Join Acknowledge Control"]
pub type HJACKCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCTL_SPEC, HJACKCTL_A, O>;
impl<'a, const O: u8> HJACKCTL_W<'a, O> {
    #[doc = "ACK the Hot-Join request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HJACKCTL_A::_0)
    }
    #[doc = "NACK and send broadcast CCC to disable Hot-Join"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HJACKCTL_A::_1)
    }
}
#[doc = "Field `ABT` reader - Abort"]
pub type ABT_R = crate::BitReader<ABT_A>;
#[doc = "Abort\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABT_A {
    #[doc = "0: I3C is running."]
    _0 = 0,
    #[doc = "1: I3C has aborted a transfer."]
    _1 = 1,
}
impl From<ABT_A> for bool {
    #[inline(always)]
    fn from(variant: ABT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABT_A {
        match self.bits {
            false => ABT_A::_0,
            true => ABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABT_A::_1
    }
}
#[doc = "Field `ABT` writer - Abort"]
pub type ABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCTL_SPEC, ABT_A, O>;
impl<'a, const O: u8> ABT_W<'a, O> {
    #[doc = "I3C is running."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABT_A::_0)
    }
    #[doc = "I3C has aborted a transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABT_A::_1)
    }
}
#[doc = "Field `RSM` reader - Resume"]
pub type RSM_R = crate::BitReader<RSM_A>;
#[doc = "Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSM_A {
    #[doc = "0: I3C is running."]
    _0 = 0,
    #[doc = "1: I3C is suspended (RW1C)."]
    _1 = 1,
}
impl From<RSM_A> for bool {
    #[inline(always)]
    fn from(variant: RSM_A) -> Self {
        variant as u8 != 0
    }
}
impl RSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSM_A {
        match self.bits {
            false => RSM_A::_0,
            true => RSM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSM_A::_1
    }
}
#[doc = "Field `RSM` writer - Resume"]
pub type RSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCTL_SPEC, RSM_A, O>;
impl<'a, const O: u8> RSM_W<'a, O> {
    #[doc = "I3C is running."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSM_A::_0)
    }
    #[doc = "I3C is suspended (RW1C)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSM_A::_1)
    }
}
#[doc = "Field `BUSE` reader - Bus Enable"]
pub type BUSE_R = crate::BitReader<BUSE_A>;
#[doc = "Bus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSE_A {
    #[doc = "0: I3C bus operation is disabled."]
    _0 = 0,
    #[doc = "1: I3C bus operation is enabled."]
    _1 = 1,
}
impl From<BUSE_A> for bool {
    #[inline(always)]
    fn from(variant: BUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSE_A {
        match self.bits {
            false => BUSE_A::_0,
            true => BUSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSE_A::_1
    }
}
#[doc = "Field `BUSE` writer - Bus Enable"]
pub type BUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCTL_SPEC, BUSE_A, O>;
impl<'a, const O: u8> BUSE_W<'a, O> {
    #[doc = "I3C bus operation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSE_A::_0)
    }
    #[doc = "I3C bus operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Include I3C Broadcast Address"]
    #[inline(always)]
    pub fn incba(&self) -> INCBA_R {
        INCBA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Hot-Join Acknowledge Control"]
    #[inline(always)]
    pub fn hjackctl(&self) -> HJACKCTL_R {
        HJACKCTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 29 - Abort"]
    #[inline(always)]
    pub fn abt(&self) -> ABT_R {
        ABT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Resume"]
    #[inline(always)]
    pub fn rsm(&self) -> RSM_R {
        RSM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus Enable"]
    #[inline(always)]
    pub fn buse(&self) -> BUSE_R {
        BUSE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Include I3C Broadcast Address"]
    #[inline(always)]
    #[must_use]
    pub fn incba(&mut self) -> INCBA_W<0> {
        INCBA_W::new(self)
    }
    #[doc = "Bit 8 - Hot-Join Acknowledge Control"]
    #[inline(always)]
    #[must_use]
    pub fn hjackctl(&mut self) -> HJACKCTL_W<8> {
        HJACKCTL_W::new(self)
    }
    #[doc = "Bit 29 - Abort"]
    #[inline(always)]
    #[must_use]
    pub fn abt(&mut self) -> ABT_W<29> {
        ABT_W::new(self)
    }
    #[doc = "Bit 30 - Resume"]
    #[inline(always)]
    #[must_use]
    pub fn rsm(&mut self) -> RSM_W<30> {
        RSM_W::new(self)
    }
    #[doc = "Bit 31 - Bus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buse(&mut self) -> BUSE_W<31> {
        BUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bctl](index.html) module"]
pub struct BCTL_SPEC;
impl crate::RegisterSpec for BCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bctl::R](R) reader structure"]
impl crate::Readable for BCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bctl::W](W) writer structure"]
impl crate::Writable for BCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCTL to value 0"]
impl crate::Resettable for BCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
