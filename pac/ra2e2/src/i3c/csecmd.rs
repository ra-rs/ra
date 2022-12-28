#[doc = "Register `CSECMD` reader"]
pub struct R(crate::R<CSECMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSECMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSECMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSECMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSECMD` writer"]
pub struct W(crate::W<CSECMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSECMD_SPEC>;
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
impl From<crate::W<CSECMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSECMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVIRQE` reader - Slave Interrupt Requests Enable"]
pub type SVIRQE_R = crate::BitReader<SVIRQE_A>;
#[doc = "Slave Interrupt Requests Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVIRQE_A {
    #[doc = "0: DISABLED: Slave-initiated Interrupts is Disabled by the Master to control."]
    _0 = 0,
    #[doc = "1: ENABLED: Slave-initiated Interrupts is Enabled by the Master to control."]
    _1 = 1,
}
impl From<SVIRQE_A> for bool {
    #[inline(always)]
    fn from(variant: SVIRQE_A) -> Self {
        variant as u8 != 0
    }
}
impl SVIRQE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVIRQE_A {
        match self.bits {
            false => SVIRQE_A::_0,
            true => SVIRQE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVIRQE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVIRQE_A::_1
    }
}
#[doc = "Field `SVIRQE` writer - Slave Interrupt Requests Enable"]
pub type SVIRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSECMD_SPEC, SVIRQE_A, O>;
impl<'a, const O: u8> SVIRQE_W<'a, O> {
    #[doc = "DISABLED: Slave-initiated Interrupts is Disabled by the Master to control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVIRQE_A::_0)
    }
    #[doc = "ENABLED: Slave-initiated Interrupts is Enabled by the Master to control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVIRQE_A::_1)
    }
}
#[doc = "Field `MSRQE` reader - Mastership Requests Enable"]
pub type MSRQE_R = crate::BitReader<MSRQE_A>;
#[doc = "Mastership Requests Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSRQE_A {
    #[doc = "0: DISABLED: Mastership requests from Secondary Masters is Disabled by the Current Master to control."]
    _0 = 0,
    #[doc = "1: ENABLED: Mastership requests from Secondary Masters is Enabled by the Current Master to control."]
    _1 = 1,
}
impl From<MSRQE_A> for bool {
    #[inline(always)]
    fn from(variant: MSRQE_A) -> Self {
        variant as u8 != 0
    }
}
impl MSRQE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSRQE_A {
        match self.bits {
            false => MSRQE_A::_0,
            true => MSRQE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSRQE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSRQE_A::_1
    }
}
#[doc = "Field `MSRQE` writer - Mastership Requests Enable"]
pub type MSRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSECMD_SPEC, MSRQE_A, O>;
impl<'a, const O: u8> MSRQE_W<'a, O> {
    #[doc = "DISABLED: Mastership requests from Secondary Masters is Disabled by the Current Master to control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSRQE_A::_0)
    }
    #[doc = "ENABLED: Mastership requests from Secondary Masters is Enabled by the Current Master to control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSRQE_A::_1)
    }
}
#[doc = "Field `HJEVE` reader - Hot-Join Event Enable"]
pub type HJEVE_R = crate::BitReader<HJEVE_A>;
#[doc = "Hot-Join Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HJEVE_A {
    #[doc = "0: DISABLED: Slave-initiated Hot-Join is Disabled by the Master to control."]
    _0 = 0,
    #[doc = "1: ENABLED: Slave-initiated Hot-Join is Enabled by the Master to control."]
    _1 = 1,
}
impl From<HJEVE_A> for bool {
    #[inline(always)]
    fn from(variant: HJEVE_A) -> Self {
        variant as u8 != 0
    }
}
impl HJEVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HJEVE_A {
        match self.bits {
            false => HJEVE_A::_0,
            true => HJEVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HJEVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HJEVE_A::_1
    }
}
#[doc = "Field `HJEVE` writer - Hot-Join Event Enable"]
pub type HJEVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSECMD_SPEC, HJEVE_A, O>;
impl<'a, const O: u8> HJEVE_W<'a, O> {
    #[doc = "DISABLED: Slave-initiated Hot-Join is Disabled by the Master to control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HJEVE_A::_0)
    }
    #[doc = "ENABLED: Slave-initiated Hot-Join is Enabled by the Master to control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HJEVE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Interrupt Requests Enable"]
    #[inline(always)]
    pub fn svirqe(&self) -> SVIRQE_R {
        SVIRQE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mastership Requests Enable"]
    #[inline(always)]
    pub fn msrqe(&self) -> MSRQE_R {
        MSRQE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Hot-Join Event Enable"]
    #[inline(always)]
    pub fn hjeve(&self) -> HJEVE_R {
        HJEVE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Interrupt Requests Enable"]
    #[inline(always)]
    #[must_use]
    pub fn svirqe(&mut self) -> SVIRQE_W<0> {
        SVIRQE_W::new(self)
    }
    #[doc = "Bit 1 - Mastership Requests Enable"]
    #[inline(always)]
    #[must_use]
    pub fn msrqe(&mut self) -> MSRQE_W<1> {
        MSRQE_W::new(self)
    }
    #[doc = "Bit 3 - Hot-Join Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hjeve(&mut self) -> HJEVE_W<3> {
        HJEVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Slave Events Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csecmd](index.html) module"]
pub struct CSECMD_SPEC;
impl crate::RegisterSpec for CSECMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csecmd::R](R) reader structure"]
impl crate::Readable for CSECMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csecmd::W](W) writer structure"]
impl crate::Writable for CSECMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSECMD to value 0"]
impl crate::Resettable for CSECMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
