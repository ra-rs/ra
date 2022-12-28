#[doc = "Register `STIPR` reader"]
pub struct R(crate::R<STIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIPR` writer"]
pub struct W(crate::W<STIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIPR_SPEC>;
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
impl From<crate::W<STIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC` reader - SYNC Status Notification Enable"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "SYNC Status Notification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: Disable notification of the STSR.SYNC state"]
    _0 = 0,
    #[doc = "1: Enable notification of the STSR.SYNC state"]
    _1 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::_0,
            true => SYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC_A::_1
    }
}
#[doc = "Field `SYNC` writer - SYNC Status Notification Enable"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STIPR_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "Disable notification of the STSR.SYNC state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNC_A::_0)
    }
    #[doc = "Enable notification of the STSR.SYNC state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNC_A::_1)
    }
}
#[doc = "Field `SYNCOUT` reader - SYNCOUT Status Notification Enable"]
pub type SYNCOUT_R = crate::BitReader<SYNCOUT_A>;
#[doc = "SYNCOUT Status Notification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOUT_A {
    #[doc = "0: Disable notification of the STSR.SYNCOUT state"]
    _0 = 0,
    #[doc = "1: Enable notification of the STSR.SYNCOUT state"]
    _1 = 1,
}
impl From<SYNCOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCOUT_A {
        match self.bits {
            false => SYNCOUT_A::_0,
            true => SYNCOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCOUT_A::_1
    }
}
#[doc = "Field `SYNCOUT` writer - SYNCOUT Status Notification Enable"]
pub type SYNCOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STIPR_SPEC, SYNCOUT_A, O>;
impl<'a, const O: u8> SYNCOUT_W<'a, O> {
    #[doc = "Disable notification of the STSR.SYNCOUT state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCOUT_A::_0)
    }
    #[doc = "Enable notification of the STSR.SYNCOUT state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCOUT_A::_1)
    }
}
#[doc = "Field `SYNTOUT` reader - SYNTOUT Status Notification Enable"]
pub type SYNTOUT_R = crate::BitReader<SYNTOUT_A>;
#[doc = "SYNTOUT Status Notification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNTOUT_A {
    #[doc = "0: Disable notification of the STSR.SYNTOUT state"]
    _0 = 0,
    #[doc = "1: Enable notification of the STSR.SYNTOUT state"]
    _1 = 1,
}
impl From<SYNTOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNTOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNTOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNTOUT_A {
        match self.bits {
            false => SYNTOUT_A::_0,
            true => SYNTOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNTOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNTOUT_A::_1
    }
}
#[doc = "Field `SYNTOUT` writer - SYNTOUT Status Notification Enable"]
pub type SYNTOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STIPR_SPEC, SYNTOUT_A, O>;
impl<'a, const O: u8> SYNTOUT_W<'a, O> {
    #[doc = "Disable notification of the STSR.SYNTOUT state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNTOUT_A::_0)
    }
    #[doc = "Enable notification of the STSR.SYNTOUT state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNTOUT_A::_1)
    }
}
#[doc = "Field `W10D` reader - W10D Status Notification Enable"]
pub type W10D_R = crate::BitReader<W10D_A>;
#[doc = "W10D Status Notification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum W10D_A {
    #[doc = "0: Disable notification of the STSR.W10D state"]
    _0 = 0,
    #[doc = "1: Enable notification of the STSR.W10D state"]
    _1 = 1,
}
impl From<W10D_A> for bool {
    #[inline(always)]
    fn from(variant: W10D_A) -> Self {
        variant as u8 != 0
    }
}
impl W10D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W10D_A {
        match self.bits {
            false => W10D_A::_0,
            true => W10D_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == W10D_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == W10D_A::_1
    }
}
#[doc = "Field `W10D` writer - W10D Status Notification Enable"]
pub type W10D_W<'a, const O: u8> = crate::BitWriter<'a, u32, STIPR_SPEC, W10D_A, O>;
impl<'a, const O: u8> W10D_W<'a, O> {
    #[doc = "Disable notification of the STSR.W10D state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(W10D_A::_0)
    }
    #[doc = "Enable notification of the STSR.W10D state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(W10D_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SYNC Status Notification Enable"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNCOUT Status Notification Enable"]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SYNTOUT Status Notification Enable"]
    #[inline(always)]
    pub fn syntout(&self) -> SYNTOUT_R {
        SYNTOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - W10D Status Notification Enable"]
    #[inline(always)]
    pub fn w10d(&self) -> W10D_R {
        W10D_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC Status Notification Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<0> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 1 - SYNCOUT Status Notification Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<1> {
        SYNCOUT_W::new(self)
    }
    #[doc = "Bit 3 - SYNTOUT Status Notification Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syntout(&mut self) -> SYNTOUT_W<3> {
        SYNTOUT_W::new(self)
    }
    #[doc = "Bit 4 - W10D Status Notification Enable"]
    #[inline(always)]
    #[must_use]
    pub fn w10d(&mut self) -> W10D_W<4> {
        W10D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STCA Status Notification Permission Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stipr](index.html) module"]
pub struct STIPR_SPEC;
impl crate::RegisterSpec for STIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stipr::R](R) reader structure"]
impl crate::Readable for STIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stipr::W](W) writer structure"]
impl crate::Writable for STIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIPR to value 0"]
impl crate::Resettable for STIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
